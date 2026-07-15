//! EventLoop: macrotask queue + logical time management.
//!
//! Microsecond precision internal clock. Timers fire when advance/sleep/tick
//! moves the clock past their deadline.
#![expect(
    clippy::expect_used,
    reason = "macro_tasks pop: guarded by peek() check"
)]
//!
//! IMPORTANT: The EventLoop does NOT execute callbacks directly.
//! It only manages the queue and clock. The binding layer handles:
//! 1. Borrow EventLoop → advance time + collect due tasks → release borrow
//! 2. Execute callbacks (EventLoop not borrowed, so callbacks can register new timers)
//! 3. Borrow again → re-enqueue intervals
//!    This avoids RefCell reentrancy panics.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// HTML spec timer clamping constants (steps 11–13 of "the setTimeout() method").
const NESTING_THRESHOLD: u32 = 5;
const MIN_CLAMP_MS: f64 = 4.0;

/// The event loop — manages logical time and a priority queue of timed tasks.
pub struct EventLoop {
    /// Current logical time in microseconds.
    pub(crate) current_us: i64,
    /// Auto-advance step in microseconds (default 4000 = 4ms).
    auto_advance_step_us: i64,
    /// Macrotask priority queue (earliest deadline first).
    pub(crate) macro_tasks: BinaryHeap<Reverse<TimedTask>>,
    /// Next timer ID counter.
    next_id: u32,
    /// IDs that were cleared during callback execution (prevents re-enqueue).
    cleared_ids: std::collections::HashSet<u32>,
    /// Nesting level of the currently executing Timeout task (HTML setTimeout
    /// algorithm). `None` when not inside a timeout callback.
    /// Used so nested setTimeout chains accumulate level and clamp after >5.
    active_timeout_nesting: Option<u32>,
}

/// A timed task in the macrotask queue.
#[derive(Debug)]
pub struct TimedTask {
    /// When this task is due (microseconds).
    pub due_us: i64,
    /// Unique timer ID (for clearTimeout/clearInterval).
    pub id: u32,
    /// The V8 function to call.
    pub callback: v8::Global<v8::Function>,
    /// What kind of task.
    pub kind: TaskKind,
    /// Extra arguments to pass to callback (setTimeout/setInterval args after delay).
    pub extra_args: Vec<v8::Global<v8::Value>>,
    /// HTML timer nesting level for this task (Timeout only; 0 otherwise).
    pub nesting_level: u32,
}

/// Task kind — determines behavior after execution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskKind {
    /// setTimeout: fires once, then removed.
    Timeout,
    /// setInterval: fires, then re-enqueued with period offset.
    Interval { period_us: i64 },
    /// requestAnimationFrame: fires once per advance step, callback receives DOMHighResTimeStamp.
    Raf { deadline_ms: f64 },
}

// Ordering for BinaryHeap (min-heap via Reverse): compare by due_us, then id.
impl PartialEq for TimedTask {
    fn eq(&self, other: &Self) -> bool {
        self.due_us == other.due_us && self.id == other.id
    }
}
impl Eq for TimedTask {}

impl PartialOrd for TimedTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimedTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.due_us
            .cmp(&other.due_us)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl EventLoop {
    /// Create a new EventLoop starting at time 0.
    pub fn new() -> Self {
        Self {
            current_us: 0,
            auto_advance_step_us: 4000, // 4ms default
            macro_tasks: BinaryHeap::new(),
            next_id: 1,
            cleared_ids: std::collections::HashSet::new(),
            active_timeout_nesting: None,
        }
    }

    /// Get current logical time in milliseconds.
    pub fn get_time_ms(&self) -> f64 {
        self.current_us as f64 / 1000.0
    }

    /// Get current logical time in microseconds.
    pub fn get_time_us(&self) -> i64 {
        self.current_us
    }

    /// Set the auto-advance step (in microseconds).
    pub fn set_auto_advance_step_us(&mut self, us: i64) {
        self.auto_advance_step_us = us.max(1);
    }

    /// Get the auto-advance step in microseconds.
    pub fn auto_advance_step_us(&self) -> i64 {
        self.auto_advance_step_us
    }

    /// Nesting level used for clamp checks (parent timeout nesting, or 0).
    pub fn timer_nesting_depth(&self) -> u32 {
        self.active_timeout_nesting.unwrap_or(0)
    }

    /// Reset the event loop to initial state.
    pub fn reset(&mut self) {
        self.current_us = 0;
        self.macro_tasks.clear();
        self.cleared_ids.clear();
        self.active_timeout_nesting = None;
    }

    /// Register a new timer. Returns the timer ID.
    pub fn add_timer(
        &mut self,
        callback: v8::Global<v8::Function>,
        delay_ms: f64,
        kind: TaskKind,
        extra_args: Vec<v8::Global<v8::Value>>,
    ) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        // HTML: nesting level = currently-running timer task's nesting level, else 0.
        // New timeout task nesting level = that + 1. Clamp when level > 5 and delay < 4ms.
        let parent_nesting = self.active_timeout_nesting.unwrap_or(0);
        let task_nesting = if matches!(kind, TaskKind::Timeout) {
            parent_nesting.saturating_add(1)
        } else {
            0
        };

        // Clamp uses the *new* task nesting level (parent+1), not parent alone.
        let effective_delay_ms = if matches!(kind, TaskKind::Timeout)
            && task_nesting > NESTING_THRESHOLD
            && delay_ms < MIN_CLAMP_MS
        {
            MIN_CLAMP_MS
        } else {
            delay_ms
        };

        let delay_us = (effective_delay_ms * 1000.0) as i64;
        let due_us = self.current_us + delay_us.max(0);

        self.macro_tasks.push(Reverse(TimedTask {
            due_us,
            id,
            callback,
            kind,
            extra_args,
            nesting_level: task_nesting,
        }));

        id
    }

    /// Remove a timer by ID. Returns true if found and removed.
    pub fn remove_timer(&mut self, id: u32) -> bool {
        let before = self.macro_tasks.len();
        let tasks: Vec<_> = self.macro_tasks.drain().collect();
        self.macro_tasks = tasks.into_iter().filter(|Reverse(t)| t.id != id).collect();
        // Also track as cleared (for intervals currently being executed)
        self.cleared_ids.insert(id);
        self.macro_tasks.len() < before
    }

    /// Get the number of pending macrotasks.
    pub fn pending_count(&self) -> usize {
        self.macro_tasks.len()
    }

    // ─── Methods for the binding layer (no callback execution) ──────────────

    /// Advance the clock. Does NOT execute tasks.
    pub fn advance_time(&mut self, ms: f64) {
        self.current_us += (ms * 1000.0) as i64;
    }

    /// Advance clock by auto_advance_step. Does NOT execute tasks.
    pub fn tick_time(&mut self, ms: f64) {
        if ms > 0.0 {
            self.current_us += (ms * 1000.0) as i64;
        } else {
            self.current_us += self.auto_advance_step_us;
        }
    }

    /// Advance clock to cover all pending tasks.
    pub fn advance_to_last_deadline(&mut self) {
        if let Some(latest) = self.macro_tasks.iter().map(|Reverse(t)| t.due_us).max() {
            if latest > self.current_us {
                self.current_us = latest;
            }
        }
    }

    /// Pop all tasks whose deadline <= current_us.
    /// Returns them for external execution.
    pub fn collect_due_tasks(&mut self) -> Vec<TimedTask> {
        let mut due = Vec::new();
        while let Some(Reverse(task)) = self.macro_tasks.peek() {
            if task.due_us > self.current_us {
                break;
            }
            // SAFETY: peek() confirmed non-empty
            due.push(self.macro_tasks.pop().expect("peek succeeded").0);
        }
        due
    }

    /// Pop one due task (for tick behavior).
    pub fn collect_one_due_task(&mut self) -> Option<TimedTask> {
        if let Some(Reverse(task)) = self.macro_tasks.peek() {
            if task.due_us <= self.current_us {
                // SAFETY: peek() confirmed non-empty
                return Some(self.macro_tasks.pop().expect("peek succeeded").0);
            }
        }
        None
    }

    /// Re-enqueue an interval task with updated deadline.
    /// Skips if the timer was cleared during callback execution.
    pub fn re_enqueue_interval(&mut self, task: &TimedTask, period_us: i64) {
        if self.cleared_ids.contains(&task.id) {
            return; // Timer was cleared during execution
        }
        self.macro_tasks.push(Reverse(TimedTask {
            due_us: task.due_us + period_us,
            id: task.id,
            callback: task.callback.clone(),
            kind: task.kind,
            extra_args: task.extra_args.clone(),
            nesting_level: task.nesting_level,
        }));
    }
}

impl Default for EventLoop {
    fn default() -> Self {
        Self::new()
    }
}

/// Execute a collected task's callback. Call this OUTSIDE the EventLoop borrow.
pub fn execute_task(scope: &v8::PinScope<'_, '_>, task: &TimedTask) {
    let func = v8::Local::new(scope, &task.callback);
    let global = scope.get_current_context().global(scope);
    let mut args: Vec<v8::Local<v8::Value>> = Vec::new();
    if let TaskKind::Raf { deadline_ms } = task.kind {
        let ts = v8::Number::new(scope, deadline_ms);
        args.push(ts.into());
    }
    for g in &task.extra_args {
        args.push(v8::Local::new(scope, g));
    }
    func.call(scope, global.into(), &args);
}

/// Execute a collected task's callback with timer nesting tracking.
/// Call this OUTSIDE the EventLoop borrow.
pub fn execute_task_tracked(
    scope: &v8::PinScope<'_, '_>,
    state: &crate::state::RuntimeState,
    task: &TimedTask,
) {
    // HTML: while this timeout runs, nested setTimeout sees this task's nesting_level.
    let prev = if matches!(task.kind, TaskKind::Timeout) {
        let mut el = state.event_loop.borrow_mut();
        let prev = el.active_timeout_nesting;
        el.active_timeout_nesting = Some(task.nesting_level);
        Some(prev)
    } else {
        None
    };
    execute_task(scope, task);
    if let Some(prev) = prev {
        state.event_loop.borrow_mut().active_timeout_nesting = prev;
    }
}

/// Execute a batch of tasks and re-enqueue intervals.
/// This is the main "run tasks" function used by the binding layer.
/// `state` is the RuntimeState — we borrow event_loop only for queue ops, not during execution.
pub fn run_due_tasks(scope: &v8::PinScope<'_, '_>, state: &crate::state::RuntimeState) {
    loop {
        // 1. Borrow → collect due tasks → release
        let due_tasks = state.event_loop.borrow_mut().collect_due_tasks();
        if due_tasks.is_empty() {
            break;
        }

        // 2. Execute each task (EventLoop NOT borrowed)
        for task in &due_tasks {
            execute_task_tracked(scope, state, task);
        }

        // 3. Borrow → re-enqueue intervals → release
        let mut el = state.event_loop.borrow_mut();
        for task in &due_tasks {
            if let TaskKind::Interval { period_us } = task.kind {
                el.re_enqueue_interval(task, period_us);
            }
        }
    }
}

/// Run one due task (for tick).
pub fn run_one_due_task(scope: &v8::PinScope<'_, '_>, state: &crate::state::RuntimeState) {
    let task = state.event_loop.borrow_mut().collect_one_due_task();
    if let Some(task) = task {
        execute_task_tracked(scope, state, &task);
        if let TaskKind::Interval { period_us } = task.kind {
            state
                .event_loop
                .borrow_mut()
                .re_enqueue_interval(&task, period_us);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_loop_initial_state() {
        let el = EventLoop::new();
        assert_eq!(el.get_time_ms(), 0.0);
        assert_eq!(el.get_time_us(), 0);
        assert_eq!(el.pending_count(), 0);
        assert_eq!(el.auto_advance_step_us(), 4000);
    }

    #[test]
    fn event_loop_set_auto_advance_step() {
        let mut el = EventLoop::new();
        el.set_auto_advance_step_us(1000);
        assert_eq!(el.auto_advance_step_us(), 1000);
    }

    #[test]
    fn event_loop_reset() {
        let mut el = EventLoop::new();
        el.current_us = 50000;
        el.active_timeout_nesting = Some(10);
        el.reset();
        assert_eq!(el.get_time_us(), 0);
        assert_eq!(el.pending_count(), 0);
        assert_eq!(el.timer_nesting_depth(), 0);
        assert!(el.active_timeout_nesting.is_none());
    }

    #[test]
    fn timer_clamp_constants_match_html_spec() {
        assert_eq!(NESTING_THRESHOLD, 5);
        assert_eq!(MIN_CLAMP_MS, 4.0);
    }

    #[test]
    fn nested_timeout_clamp_when_task_nesting_exceeds_five() {
        // parent nesting 5 → new task level 6 > 5 → clamp 0ms to 4ms.
        let parent = 5u32;
        let task_nesting = parent.saturating_add(1);
        let delay_ms = 0.0;
        let effective = if task_nesting > NESTING_THRESHOLD && delay_ms < MIN_CLAMP_MS {
            MIN_CLAMP_MS
        } else {
            delay_ms
        };
        assert_eq!(task_nesting, 6);
        assert_eq!(effective, 4.0);
    }

    #[test]
    fn nesting_five_does_not_clamp() {
        // parent 4 → task level 5 is not > 5 → no clamp (HTML strict > 5).
        let parent = 4u32;
        let task_nesting = parent.saturating_add(1);
        let delay_ms = 0.0;
        let effective = if task_nesting > NESTING_THRESHOLD && delay_ms < MIN_CLAMP_MS {
            MIN_CLAMP_MS
        } else {
            delay_ms
        };
        assert_eq!(task_nesting, 5);
        assert_eq!(effective, 0.0);
    }

    #[test]
    fn top_level_timeout_has_nesting_one_no_clamp() {
        let parent = 0u32;
        let task_nesting = parent.saturating_add(1);
        let delay_ms = 0.0;
        let effective = if task_nesting > NESTING_THRESHOLD && delay_ms < MIN_CLAMP_MS {
            MIN_CLAMP_MS
        } else {
            delay_ms
        };
        assert_eq!(task_nesting, 1);
        assert_eq!(effective, 0.0);
    }
}

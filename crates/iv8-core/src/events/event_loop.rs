//! EventLoop: macrotask queue + logical time management.
//!
//! Microsecond precision internal clock. Timers fire when advance/sleep/tick
//! moves the clock past their deadline.
//!
//! IMPORTANT: The EventLoop does NOT execute callbacks directly.
//! It only manages the queue and clock. The binding layer handles:
//! 1. Borrow EventLoop → advance time + collect due tasks → release borrow
//! 2. Execute callbacks (EventLoop not borrowed, so callbacks can register new timers)
//! 3. Borrow again → re-enqueue intervals
//! This avoids RefCell reentrancy panics.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

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
}

/// Task kind — determines behavior after execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskKind {
    /// setTimeout: fires once, then removed.
    Timeout,
    /// setInterval: fires, then re-enqueued with period offset.
    Interval { period_us: i64 },
    /// requestAnimationFrame: fires once per advance step.
    Raf,
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

    /// Reset the event loop to initial state.
    pub fn reset(&mut self) {
        self.current_us = 0;
        self.macro_tasks.clear();
        self.cleared_ids.clear();
    }

    /// Register a new timer. Returns the timer ID.
    pub fn add_timer(
        &mut self,
        callback: v8::Global<v8::Function>,
        delay_ms: f64,
        kind: TaskKind,
    ) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let delay_us = (delay_ms * 1000.0) as i64;
        let due_us = self.current_us + delay_us.max(0);

        self.macro_tasks.push(Reverse(TimedTask {
            due_us,
            id,
            callback,
            kind,
        }));

        id
    }

    /// Remove a timer by ID. Returns true if found and removed.
    pub fn remove_timer(&mut self, id: u32) -> bool {
        let before = self.macro_tasks.len();
        let tasks: Vec<_> = self.macro_tasks.drain().collect();
        self.macro_tasks = tasks
            .into_iter()
            .filter(|Reverse(t)| t.id != id)
            .collect();
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
            due.push(self.macro_tasks.pop().expect("peek succeeded").0);
        }
        due
    }

    /// Pop one due task (for tick behavior).
    pub fn collect_one_due_task(&mut self) -> Option<TimedTask> {
        if let Some(Reverse(task)) = self.macro_tasks.peek() {
            if task.due_us <= self.current_us {
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
    let undefined = v8::undefined(scope);
    func.call(scope, undefined.into(), &[]);
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
            execute_task(scope, task);
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
        execute_task(scope, &task);
        if let TaskKind::Interval { period_us } = task.kind {
            state.event_loop.borrow_mut().re_enqueue_interval(&task, period_us);
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
        el.reset();
        assert_eq!(el.get_time_us(), 0);
        assert_eq!(el.pending_count(), 0);
    }
}

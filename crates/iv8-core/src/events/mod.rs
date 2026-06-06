//! EventLoop + TimeSystem: logical/system time + macrotask queue.
//!
//! Implements the 6 eventLoop API methods:
//! - advance(ms, step_ms=16.67)
//! - sleep(ms)
//! - tick(ms=0)
//! - drain()
//! - drainMicrotasks()
//! - drainTimers()
//!
//! Plus timer registration (setTimeout/setInterval) and rAF queue.

pub mod binding;
pub mod date_interceptor;
pub mod event_loop;
pub mod input_sim;
pub mod page_api;
pub mod target;
pub mod timers;

pub use event_loop::{EventLoop, TaskKind, TimedTask};
pub use target::EventListenerRegistry;

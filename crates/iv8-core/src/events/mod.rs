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

pub mod event_loop;
pub mod binding;
pub mod timers;
pub mod date_interceptor;
pub mod target;
pub mod page_api;
pub mod input_sim;

pub use event_loop::{EventLoop, TaskKind, TimedTask};
pub use target::EventListenerRegistry;

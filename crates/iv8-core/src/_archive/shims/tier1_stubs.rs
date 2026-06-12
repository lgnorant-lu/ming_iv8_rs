//! Tier 1: Browser API surface stubs.
//!
//! 716 empty constructor functions that make `typeof API === 'function'` checks pass.
//! All are configurable+writable so users can override them.
//!
//! Generated from the diff between iv8 0.1.2 globals and iv8-rs globals.

pub const TIER1_STUBS_JS: &str = include_str!("tier1_stubs.js");

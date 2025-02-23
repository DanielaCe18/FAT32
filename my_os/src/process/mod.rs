//! Process management module.
//! This module handles process creation, states, and context switching.

pub mod context;
pub mod process;

pub use context::switch_context;
pub use process::{Process, ProcessState};


use core::sync::atomic::{AtomicU64, Ordering};

/// Process states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Ready,
    Running,
    Waiting,
    Terminated,
}

/// Represents a single process.
#[derive(Debug)]
pub struct Process {
    pub pid: u64,
    pub state: ProcessState,
    pub name: &'static str,
}

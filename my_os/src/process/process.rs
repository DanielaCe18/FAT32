use crate::println;
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

impl Process {
    pub fn new(name: &'static str) -> Self {
        static NEXT_PID: AtomicU64 = AtomicU64::new(1);
        let pid = NEXT_PID.fetch_add(1, Ordering::Relaxed);

        Process {
            pid,
            state: ProcessState::Ready,
            name,
        }
    }

    /// Mark the process as running.
    pub fn run(&mut self) {
        self.state = ProcessState::Running;
        println!("Process {} (PID: {}) is running", self.name, self.pid);
    }

    /// Mark the process as completed.
    pub fn terminate(&mut self) {
        self.state = ProcessState::Terminated;
        println!("Process {} (PID: {}) terminated", self.name, self.pid);
    }
}


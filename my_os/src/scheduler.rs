use core::task::Waker;
use alloc::collections::VecDeque;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;
use crate::process::{Process, ProcessState};

/// Global scheduler instance.
lazy_static! {
    pub static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());
}

/// Represents a process with its state and priority.
#[derive(Debug)]
pub struct ScheduledProcess {
    pub process: Process,
    pub waker: Option<Waker>,
}

#[derive(Debug)]
pub struct Scheduler {
    queue: VecDeque<ScheduledProcess>,
    next_pid: AtomicU64,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            queue: VecDeque::new(),
            next_pid: AtomicU64::new(1),
        }
    }

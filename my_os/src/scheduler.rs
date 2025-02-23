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

    /// Add a new process to the scheduler.
    pub fn add_process(&mut self, process: Process) {
        let scheduled = ScheduledProcess { process, waker: None };
        self.queue.push_back(scheduled);
    }

    /// Get the next process to run (Round-Robin).
    pub fn next_process(&mut self) -> Option<ScheduledProcess> {
        while let Some(mut scheduled) = self.queue.pop_front() {
            // Only return if the process is ready to run.
            if scheduled.process.state == ProcessState::Ready {
                scheduled.process.state = ProcessState::Running;
                return Some(scheduled);
            } else {
                // If not ready, push back and check the next one.
                self.queue.push_back(scheduled);
            }
        }
        None
    }

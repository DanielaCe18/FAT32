use core::task::Waker;
use alloc::collections::VecDeque;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;
use crate::process::{Process, ProcessState};
use lazy_static::lazy_static;

// Global scheduler instance protected by a mutex for thread safety.
lazy_static! {
    pub static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());
}

#[derive(Debug)]
pub struct ScheduledProcess {
    pub process: Process,       // The process being scheduled.
    pub waker: Option<Waker>,   // Optional waker for async tasks.
}

#[derive(Debug)]
pub struct Scheduler {
    queue: VecDeque<ScheduledProcess>, // Queue to hold scheduled processes.
    next_pid: AtomicU64,               // Atomic counter for unique process IDs.
}

impl Scheduler {
    // Creates a new scheduler with an empty queue and PID counter starting at 1.
    pub fn new() -> Self {
        Scheduler {
            queue: VecDeque::new(),
            next_pid: AtomicU64::new(1),
        }
    }

    // Adds a new process to the scheduling queue.
    pub fn add_process(&mut self, process: Process) {
        let scheduled = ScheduledProcess { process, waker: None };
        self.queue.push_back(scheduled);
    }

    // Fetches the next ready process for execution.
    pub fn next_process(&mut self) -> Option<ScheduledProcess> {
        while let Some(mut scheduled) = self.queue.pop_front() {
            if scheduled.process.state == ProcessState::Ready {
                scheduled.process.state = ProcessState::Running; // Mark as running.
                return Some(scheduled);
            } else {
                self.queue.push_back(scheduled); // Requeue non-ready processes.
            }
        }
        None
    }

    // Requeues a process unless it has terminated.
    pub fn complete_process(&mut self, process: Process) {
        if process.state != ProcessState::Terminated {
            let scheduled = ScheduledProcess { process, waker: None };
            self.queue.push_back(scheduled);
        }
    }

    // Generates a unique process ID using atomic operations.
    pub fn generate_pid(&self) -> u64 {
        self.next_pid.fetch_add(1, Ordering::Relaxed)
    }
}

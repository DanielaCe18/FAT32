use core::task::Waker;
use alloc::collections::VecDeque;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;
use crate::process::{Process, ProcessState};
use lazy_static::lazy_static;


lazy_static! {
    pub static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());
}

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

    pub fn add_process(&mut self, process: Process) {
        let scheduled = ScheduledProcess { process, waker: None };
        self.queue.push_back(scheduled);
    }

    pub fn next_process(&mut self) -> Option<ScheduledProcess> {
        while let Some(mut scheduled) = self.queue.pop_front() {
            if scheduled.process.state == ProcessState::Ready {
                scheduled.process.state = ProcessState::Running;
                return Some(scheduled);
            } else {
                self.queue.push_back(scheduled);
            }
        }
        None
    }

    pub fn complete_process(&mut self, process: Process) {
        if process.state != ProcessState::Terminated {
            let scheduled = ScheduledProcess { process, waker: None };
            self.queue.push_back(scheduled);
        }
    }

    pub fn generate_pid(&self) -> u64 {
        self.next_pid.fetch_add(1, Ordering::Relaxed)
    }
}


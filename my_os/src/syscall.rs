use crate::memory::{allocate, deallocate};
use crate::process::Process;
use core::ptr;

/// Syscall: Allocate Memory
pub fn syscall_alloc(size: usize) -> *mut u8 {
    let ptr = allocate(size);
    if ptr.is_null() {
        panic!("Syscall failed: Unable to allocate {} bytes", size);
    }
    ptr
}

/// Syscall: Deallocate Memory
pub fn syscall_dealloc(ptr: *mut u8, size: usize) {
    if ptr.is_null() {
        panic!("Syscall failed: Invalid pointer for deallocation");
    }
    deallocate(ptr, size);
}

/// Syscall: Create Process
pub fn syscall_create_process(name: &'static str, stack_ptr: usize) -> Process {
    let process = Process::new(name, stack_ptr);
    process
}

/// Syscall: Terminate Process
pub fn syscall_terminate_process(process: &mut Process) {
    process.terminate();
}

/// Syscall: Read Memory (Example)
pub fn syscall_read_mem(ptr: *const u8, size: usize) -> Option<&'static [u8]> {
    if ptr.is_null() {
        return None;
    }
    Some(unsafe { core::slice::from_raw_parts(ptr, size) })
}


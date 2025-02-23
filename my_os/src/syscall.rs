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

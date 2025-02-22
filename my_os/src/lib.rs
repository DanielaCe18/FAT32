#![no_std]  // This must be the first line to disable the standard library
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod attribute;
pub mod datetime;
pub mod name;
pub mod offset_iter;
pub mod directory;
pub mod filesystem;
pub mod memory;
pub mod process;
pub mod scheduler;
pub mod syscall;

// Allocation error handler for kernel mode
#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}


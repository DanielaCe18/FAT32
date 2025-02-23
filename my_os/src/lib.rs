#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

// Modules
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
pub mod slab;  // Ensure slab is included if you made it a separate file

// Global allocator
use crate::slab::GlobalAllocator;
#[global_allocator]
static ALLOCATOR: GlobalAllocator = GlobalAllocator;

// Allocation error handler
#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}


#![cfg_attr(not(feature = "std"), no_std)]  // Enable no_std if std feature is disabled
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
pub mod slab;   // Add the Slab allocator module here

#[cfg(feature = "global_alloc")]
pub use slab::ALLOCATOR;  // Use the allocator when the feature is enabled

#[cfg(not(feature = "std"))]
#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}


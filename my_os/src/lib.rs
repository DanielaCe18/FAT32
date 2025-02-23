#![no_std]
#![feature(alloc_error_handler)]
#[macro_use]
extern crate alloc;

use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

// Global serial port for printing
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

// Modules
pub mod directory;
pub mod filesystem;
pub mod memory;
pub mod process;
pub mod scheduler;
pub mod syscall;
pub mod slab;

// Print macros for global access
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!(*crate::SERIAL1.lock(), $($arg)*);
    }};
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        $crate::print!("{}\n", format_args!($($arg)*));
    }};
}

// Global allocator
use crate::slab::GlobalAllocator;

#[global_allocator]
static ALLOCATOR: GlobalAllocator = GlobalAllocator;

// Allocation error handler
#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}


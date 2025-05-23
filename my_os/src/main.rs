#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;
use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;
use my_os::process::Process; // Correct import from crate root

// Serial port for printing
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

// Print macros
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!(*SERIAL1.lock(), $($arg)*);
    }};
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        print!("{}\n", format_args!($($arg)*));
    }};
}

// Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to My OS!");

    // Example process creation
    let mut process = Process::new("Init");  
    process.run();
    process.terminate();

    loop {}
}

// Panic handler for kernel mode (no_std)
#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel Panic: {:?}", info);
    loop {}
}

// Allocation error handler (no_std only)
#[cfg(not(feature = "std"))]
#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}

// Panic handler for testing mode (std)
#[cfg(all(test, feature = "std"))]
#[panic_handler]
fn test_panic(info: &PanicInfo) -> ! {
    println!("Test Panic: {:?}", info);
    loop {}
}


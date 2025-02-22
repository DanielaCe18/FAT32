#![no_std]
#![no_main]

use core::panic::PanicInfo;
use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

// Serial port for printing
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

// Print macro for serial output
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!(*SERIAL1.lock(), $($arg)*);
    }};
}

// Println macro for serial output
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
    loop {}
}

// Panic handler for kernel mode (no_std)
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel Panic: {:?}", info);
    loop {}
}

// Panic handler for testing mode (with std)
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Test Panic: {:?}", info);
    loop {}
}


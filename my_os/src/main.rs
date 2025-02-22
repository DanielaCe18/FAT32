#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Entry point of the kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to My OS!");
    loop {}
}

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


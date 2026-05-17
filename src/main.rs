#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod kernel;
use core::panic::PanicInfo;
use crate::kernel::kernel_init;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    println!("Initialising the kernel");

    kernel_init();

    loop {
    }
}

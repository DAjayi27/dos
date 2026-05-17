use core::arch::asm;
use crate::kernel::interrupts::init_idt;

pub mod vga;

pub mod writer;
mod colors;
pub mod serial;
pub mod interrupts;

/// Initializes the kernel.
    ///
    /// This function is intended to perform any necessary setup or
    /// initialization tasks required for the kernel to operate.
    /// Currently, it is a placeholder and does not perform any actions.
pub fn kernel_init(){

    init_idt();


    unsafe {
        asm!("int 0x03",options(nomem, nostack));
    }


}
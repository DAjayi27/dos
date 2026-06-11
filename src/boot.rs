use core::arch::global_asm;

global_asm!(
    // Place this in the special .text.boot section
    // so the linker puts it first at 0x00080000
    ".section .text.boot",

    // _start is what the linker script's ENTRY() refers to
    // and what the CPU jumps to at boot
    "_start:",

    // On a real Pi 4, all 4 CPU cores start simultaneously.
    // We only want core 0 to run our kernel.
    // Read the core ID from the MPIDR_EL1 system register.
    "   mrs x0, mpidr_el1",
    "   ubfx x0, x0, #8, #4",  // extract bits [11:8], 4 bits wide READ: https://developer.arm.com/documentation/100403/0200/register-descriptions/aarch64-system-registers/mpidr-el1--multiprocessor-affinity-register--el1
    "   cbnz x0, .halt",     // if core ID != 0, go to halt

    // Set up the stack pointer.
    // __stack_top comes from our linker script.
    "   ldr x0, =__stack_top",
    "   mov sp, x0",

    // Zero out the BSS section.
    // __bss_start and __bss_end also come from the linker script.
    // NOTE: THE BSS SECTION IS FOR ALL UN INITIALISE DATA. SO IT'S BETTER TO ZERO IT NOW
    "   ldr x0, =__bss_start",
    "   ldr x1, =__bss_end",
    ".zero_bss:",
    "   cmp x0, x1",
    "   b.ge .done_bss",
    "   str xzr, [x0], #8",  // write 8 zero bytes, advance pointer
    "   b .zero_bss",
    ".done_bss:",

    // Jump into Rust
    "   bl kernel_main",

    // If kernel_main ever returns (it shouldn't), halt
    ".halt:",
    "   wfe",               // Wait For Event — low power idle
    "   b .halt",
);
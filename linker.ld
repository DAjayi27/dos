ENTRY(_start)

SECTIONS
{
    . = 0x00080000;

    .text :
    {
        KEEP(*(.text.boot)) /*ensure the boot asm is run first*/
        *(.text*)
    }

    .rodata : { *(.rodata*) }

    .data : { *(.data*) }

    .bss :
    {
        __bss_start = .;
        *(.bss*)
        __bss_end = .;
    }

    . = ALIGN(16); /*Round the stack to the next multiple of 16*/
    . = . + 0x4000; /*add a 16 kb (0x4000 bytes) empty space */
    __stack_top = .; /* create the named addr pointing to the top of the stack (this is called by the boot assembly)*/
}
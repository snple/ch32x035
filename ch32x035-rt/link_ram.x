/* CH32X035 Linker Script for ch32x035-rt — run-from-ram variant
 *
 * Places the entire image (including the vector table and code) in RAM.
 * Used when flashing via a bootloader (e.g. the CH32X035 USB/serial
 * bootloader) that loads the image into RAM and jumps to it.
 */

INCLUDE memory.x

PROVIDE(DefaultHandler = DefaultHandler_);

INCLUDE device.x

ENTRY(_start);

EXTERN(__RESET_VECTOR);
EXTERN(Reset);
EXTERN(__EXCEPTIONS);
EXTERN(DefaultHandler);

/* Core exception handlers (can be overridden by the user via #[exception]) */
PROVIDE(NonMaskableInt = DefaultHandler);
PROVIDE(HardFault = HardFault_);
PROVIDE(EcallMMode = DefaultHandler);
PROVIDE(EcallUMode = DefaultHandler);
PROVIDE(BreakPoint = DefaultHandler);
PROVIDE(SysTick = DefaultHandler);
PROVIDE(Software = DefaultHandler);

EXTERN(__INTERRUPTS);

PROVIDE(__pre_init = DefaultPreInit);

SECTIONS
{
    PROVIDE(_ram_start = ORIGIN(RAM));
    PROVIDE(_ram_end = ORIGIN(RAM) + LENGTH(RAM));
    PROVIDE(_stack_start = _ram_end);

    /* Reset entry - first bytes of the image in RAM */
    .init ORIGIN(RAM) :
    {
        KEEP(*(.init));
        . = ALIGN(4);
    } > RAM

    /* Vector table - mtvec points here */
    .vector_table :
    {
        __vector_table = .;
        KEEP(*(.vector_table.reset_vector));
        __exceptions = .;
        KEEP(*(.vector_table.exceptions));
        __eexceptions = .;
        KEEP(*(.vector_table.interrupts));
    } > RAM

    /* Code Section */
    .text : ALIGN(4)
    {
        __stext = .;
        *(.text .text.*);
        . = ALIGN(4);
        __etext = .;
    } > RAM

    /* Read-only Data */
    .rodata : ALIGN(4)
    {
        __srodata = .;
        *(.rodata .rodata.*);
        *(.srodata .srodata.*);
        . = ALIGN(4);
        __erodata = .;
    } > RAM

    /* RAM Functions */
    .ramfunc : ALIGN(4)
    {
        __sramfunc = .;
        *(.ramfunc .ramfunc.*);
        . = ALIGN(4);
        __eramfunc = .;
    } > RAM AT>RAM
    __siramfunc = LOADADDR(.ramfunc);

    /* Initialized Data */
    .data : ALIGN(4)
    {
        __sdata = .;
        *(.data .data.*);
        . = ALIGN(4);
        __edata = .;
    } > RAM AT>RAM
    __sidata = LOADADDR(.data);

    /* Uninitialized Data - zeroed at startup */
    .bss (NOLOAD) : ALIGN(4)
    {
        __sbss = .;
        *(.bss .bss.*);
        *(COMMON);
        . = ALIGN(4);
        __ebss = .;
    } > RAM

    /* Small Data (gp-relative) */
    .sdata : ALIGN(4)
    {
        __ssdata = .;
        *(.sdata .sdata.*);
        . = ALIGN(4);
        __esdata = .;
    } > RAM AT>RAM
    __sisdata = LOADADDR(.sdata);

    /* Small BSS (gp-relative) */
    .sbss (NOLOAD) : ALIGN(4)
    {
        __ssbss = .;
        *(.sbss .sbss.*);
        *(.scommon);
        . = ALIGN(4);
        __esbss = .;
    } > RAM

    PROVIDE(__global_pointer$ = __esbss + 0x800);

    /* Explicitly Uninitialized Data */
    .uninit (NOLOAD) : ALIGN(4)
    {
        __suninit = .;
        *(.uninit .uninit.*);
        . = ALIGN(4);
        __euninit = .;
    } > RAM

    .got (NOLOAD) :
    {
        KEEP(*(.got .got.*));
    }

    . = ALIGN(4);
    PROVIDE(__sheap = .);
    PROVIDE(_stack_end = .);

    /DISCARD/ :
    {
        *(.riscv.attributes);
        *(.comment);
    }
}

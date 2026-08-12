/* CH32X035 Linker Script for ch32x035-rt
 *
 * This linker script follows the WCH flash/RAM layout for the CH32X035:
 *   - FLASH at 0x00000000 (code + vector table)
 *   - RAM at 0x20000000 (.data, .bss, stack)
 *
 * It handles:
 *   - The RISC-V boot entry (`.init` with `_start: la sp, ...; j Reset`)
 *   - The WCH vector table (entry 0 = reset handler, 1-15 core exceptions,
 *     16-54 external interrupts)
 *   - Code and data sections
 *   - RAM initialization (.data, .bss, .ramfunc)
 *   - Stack configuration (grows down from top of RAM)
 *
 * Usage: `memory.x` (defining FLASH and RAM) must be provided by the
 * application (see ch32x035-examples). `device.x` is provided by
 * `ch32x035-pac` via `cargo:rustc-link-search`.
 */

/* Include the memory layout (provided by application or BSP) */
INCLUDE memory.x

/* `DefaultHandler_` is the runtime fallback implementation.
 * `DefaultHandler` is the public weak alias used by this script and device.x.
 * A user-defined `DefaultHandler` function overrides this alias.
 */
PROVIDE(DefaultHandler = DefaultHandler_);

/* Include device-specific interrupt handlers (provided by PAC). */
INCLUDE device.x

/* Entry point - the first instruction at flash base */
ENTRY(_start);

/* External symbols that must be provided */
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

/* Device interrupts (provided by the PAC's device.x) */
EXTERN(__INTERRUPTS);

/* Pre-init function (optional, for assembly-level early init) */
PROVIDE(__pre_init = DefaultPreInit);

SECTIONS
{
    PROVIDE(_ram_start = ORIGIN(RAM));
    PROVIDE(_ram_end = ORIGIN(RAM) + LENGTH(RAM));
    PROVIDE(_stack_start = _ram_end);

    /* ================================================================
     * FLASH Sections
     * ================================================================ */

    /* Reset entry - must be the first bytes of flash.
     * Contains `_start: la sp, _stack_start; j Reset` (from global_asm). */
    .init ORIGIN(FLASH) :
    {
        KEEP(*(.init));
        . = ALIGN(4);
    } > FLASH

    /* Vector Table - mtvec points here.
     * Entry 0: reset handler address.
     * Entries 1-15: core exceptions (__EXCEPTIONS).
     * Entries 16-54: external interrupts (__INTERRUPTS from the PAC).
     */
    .vector_table :
    {
        __vector_table = .;

        /* Reset vector (entry 0) */
        KEEP(*(.vector_table.reset_vector));

        /* Core exception vectors (entries 1-15) */
        __exceptions = .;
        KEEP(*(.vector_table.exceptions));
        __eexceptions = .;

        /* Device-specific interrupt vectors (entries 16-54) */
        KEEP(*(.vector_table.interrupts));
    } > FLASH

    /* Code Section */
    .text : ALIGN(4)
    {
        __stext = .;
        *(.text .text.*);
        . = ALIGN(4);
        __etext = .;
    } > FLASH

    /* Read-only Data */
    .rodata : ALIGN(4)
    {
        __srodata = .;
        *(.rodata .rodata.*);
        . = ALIGN(4);
        __erodata = .;
    } > FLASH

    /* ================================================================
     * RAM Functions (.ramfunc) — code loaded into RAM at startup
     * ================================================================
     *
     * Functions marked with #[ramfunc] are placed here. The linker
     * allocates space for them in RAM (VMA) but stores the initial
     * content in FLASH (LMA). The startup code copies them to RAM
     * before calling main().
     */
    .ramfunc : ALIGN(4)
    {
        __sramfunc = .;
        *(.ramfunc .ramfunc.*);
        . = ALIGN(4);
        __eramfunc = .;
    } > RAM AT>FLASH
    __siramfunc = LOADADDR(.ramfunc);

    /* ================================================================
     * RAM Sections
     * ================================================================ */

    /* Initialized Data - copied from FLASH to RAM at startup */
    .data : ALIGN(4)
    {
        __sdata = .;
        *(.data .data.*);
        . = ALIGN(4);
        __edata = .;
    } > RAM AT>FLASH
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

    /* Explicitly Uninitialized Data - NOT zeroed at startup */
    .uninit (NOLOAD) : ALIGN(4)
    {
        __suninit = .;
        *(.uninit .uninit.*);
        . = ALIGN(4);
        __euninit = .;
    } > RAM

    /* Dynamic relocations check — must precede heap/stack boundary */
    .got (NOLOAD) :
    {
        KEEP(*(.got .got.*));
    }

    /* Heap starts here */
    . = ALIGN(4);
    PROVIDE(__sheap = .);
    PROVIDE(_stack_end = .);

    /* Discard unused sections */
    /DISCARD/ :
    {
        *(.riscv.attributes);
        *(.comment);
    }
}

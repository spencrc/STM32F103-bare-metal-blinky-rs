EXTERN(__RESET)
EXTERN(Reset)
ENTRY(Reset)

EXTERN(__VECTORS)

EXTERN(DefaultHandler)

INCLUDE memory.ld
INCLUDE alias.ld

_stack_start = ORIGIN(SRAM) + LENGTH(SRAM);

SECTIONS
{
    .vector_table ORIGIN(FLASH) :
    {
        LONG(_stack_start);
        KEEP(*(.vector_table.reset));
        KEEP(*(.vector_table.vectors));
    } > FLASH

    .text :
    {
        . = ALIGN(4);
        *(.text .text.*)
        *(.rodata .rodata.*)
        . = ALIGN(4);
        _etext = .;
    }> FLASH

    .data :
    {
        . = ALIGN(4);
        _sdata = .;
        *(.data .data.*)
        . = ALIGN(4);
        _edata = .;
    }> SRAM AT> FLASH

    .bss :
    {
        . = ALIGN(4);
        _sbss = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4);
        _ebss = .;
    }> SRAM

    /DISCARD/ :
    {
        *(.ARM.exidx .ARM.exidx.*);
        *(.ARM.extab .ARM.extab.*);
    }
}
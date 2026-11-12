ENTRY(Reset_handler)

MEMORY
{
    FLASH(rx): ORIGIN = 0x08000000, LENGTH = 64K
    SRAM(rwx): ORIGIN = 0x20000000, LENGTH = 20K
}

_sram_start = ORIGIN(SRAM);
_sram_end = ORIGIN(SRAM) + LENGTH(SRAM);
_stack_start = _sram_end;

SECTIONS
{
    .text :
    {
        . = ALIGN(4);
        KEEP(*(.vector_table))
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
        _edata = .;
        . = ALIGN(4);
    }> SRAM AT> FLASH

    .bss :
    {
        . = ALIGN(4);
        _sbss = .;
        *(.bss .bss.*)
        *(COMMON)
        _ebss = .;
        . = ALIGN(4);
    }> SRAM

    /DISCARD/ :
    {
        *(.ARM.exidx .ARM.exidx.*);
        *(.ARM.extab .ARM.extab.*);
    }
}
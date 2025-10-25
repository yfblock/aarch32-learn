OUTPUT_ARCH(armv7)
ENTRY(_start)

BASE_ADDRESS = 0x40010000;

SECTIONS
{
    . = BASE_ADDRESS;
    _skernel = .;

    .text ALIGN(4K): {
        stext = .;
        *(.text.entry)
        *(.text .text.*)
        etext = .;
    }

    .rodata ALIGN(4K): {
        srodata = .;
        *(.rodata .rodata.*)
        erodata = .;
    }

    .got : {
        *(.got .got.*)
    }

    .data ALIGN(4K): {
        _sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        _edata = .;
    }

    .bss ALIGN(4K): {
        _load_end = .;
        _sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        _ebss = .;
    }

    PROVIDE(_end = .);
    /DISCARD/ : {
        *(.comment) *(.gnu*) *(.note*) *(.eh_frame*)
    }
}

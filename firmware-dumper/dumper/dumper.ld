SECTIONS {
    .text 0x00 : {
        *(.text.startup)
    }

    .dynamic 0x80 : {
        *(.dynamic)
    }

    .text : {
        *(.text.*)
    }

    .bss : {
        *(.bss.*)
    }

    .relro : {
        *(.rela.dyn)
    }

    .rodata : {
        *(.rodata.*)
    }

    /DISCARD/ : {
        *(.dynsym)
        *(.gnu.hash)
        *(.hash)
        *(.dynstr)
    }
}
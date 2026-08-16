OUTPUT_ARCH(riscv)
ENTRY(_start)

/* QEMU virt machine loads the -bios image at 0x8000_0000 and starts
   the hart there directly in M-mode. */
BASE_ADDRESS = 0x80000000;

SECTIONS
{
    . = BASE_ADDRESS;

    .text : {
        *(.text._start)
        *(.text .text.*)
    }

    .rodata : ALIGN(8) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(8) {
        *(.data .data.*)
    }

    .bss : ALIGN(8) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    . = ALIGN(16);
    . += 0x10000; /* 64 KiB M-mode stack */
    _stack_top = .;

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
    }
}

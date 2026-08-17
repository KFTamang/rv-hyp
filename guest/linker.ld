OUTPUT_ARCH(riscv)
ENTRY(_start)

/* Must match GUEST_ENTRY in src/src/main.rs: the hypervisor mret's into
   this address, and QEMU's -kernel loader places the ELF's PT_LOAD
   segments at whatever addresses this script assigns, so the two must
   agree. 0x8020_0000 is the conventional RISC-V "kernel after firmware"
   load address on QEMU virt (also OpenSBI's default FW_JUMP target). */
BASE_ADDRESS = 0x80200000;

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
    . += 0x10000; /* 64 KiB S-mode stack */
    _stack_top = .;

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
    }
}

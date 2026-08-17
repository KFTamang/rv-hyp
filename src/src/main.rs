#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

/// Physical address the guest kernel's linker script places it at, and
/// where QEMU's -kernel loader deposits build/guest_kernel.elf.
const GUEST_ENTRY: usize = 0x8020_0000;

global_asm!(
    r#"
    .section .text._start
    .global _start
_start:
    /* Only the boot hart (0) proceeds; QEMU starts every hart at this
       same reset vector, and they would otherwise race on _stack_top
       and the UART. */
    csrr t0, mhartid
    bnez t0, park

    la sp, _stack_top

    /* zero .bss */
    la t0, __bss_start
    la t1, __bss_end
1:
    bgeu t0, t1, 2f
    sd zero, 0(t0)
    addi t0, t0, 8
    j 1b
2:
    /* Safety net: route any unexpected M-mode trap to the park loop
       instead of falling through to whatever garbage mtvec defaults to. */
    la t0, park
    csrw mtvec, t0

    call rust_main

    /* rust_main does not return, but land here defensively. */
park:
    wfi
    j park
"#
);

const UART_BASE: usize = 0x1000_0000;
const UART_LSR_OFFSET: usize = 5;
const UART_LSR_THR_EMPTY: u8 = 1 << 5;

fn uart_putchar(c: u8) {
    unsafe {
        let uart = UART_BASE as *mut u8;
        while uart.add(UART_LSR_OFFSET).read_volatile() & UART_LSR_THR_EMPTY == 0 {}
        uart.write_volatile(c);
    }
}

fn uart_puts(s: &str) {
    for b in s.bytes() {
        if b == b'\n' {
            uart_putchar(b'\r');
        }
        uart_putchar(b);
    }
}

/// Grants S/U-mode full read/write/execute access to the entire address
/// space via a single NAPOT PMP region. Without this, S-mode has no
/// memory access at all and the guest faults on its first instruction.
fn pmp_allow_all() {
    unsafe {
        core::arch::asm!(
            "li t0, -1",
            "csrw pmpaddr0, t0",
            "li t0, 0x1F", // A=NAPOT(3), X=1, W=1, R=1
            "csrw pmpcfg0, t0",
            out("t0") _,
        );
    }
}

/// Sets mstatus.MPP=S and mepc=entry, then mret's into the guest.
fn enter_smode(entry: usize) -> ! {
    unsafe {
        core::arch::asm!(
            "li t0, 0x1800",   // MPP mask (bits 11-12)
            "csrc mstatus, t0",
            "li t0, 0x0800",   // MPP = S (0b01 << 11)
            "csrs mstatus, t0",
            "csrw mepc, {entry}",
            "mret",
            entry = in(reg) entry,
            options(noreturn),
        );
    }
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    uart_puts("Hello, world from M-mode!\n");

    pmp_allow_all();

    uart_puts("Jumping to S-mode guest...\n");
    enter_smode(GUEST_ENTRY);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

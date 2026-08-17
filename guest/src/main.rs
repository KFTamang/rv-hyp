#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(
    r#"
    .section .text._start
    .global _start
_start:
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
    call rust_main

3:
    wfi
    j 3b
"#
);

// The hypervisor grants S-mode a PMP region covering all of memory
// (including this MMIO range) before jumping here, so direct UART
// access works even though this code runs in S-mode.
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

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    uart_puts("Hello from S-mode guest\n");

    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

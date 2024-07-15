#![no_main]
#![no_std]
#![feature(asm_const)]

use core::panic::PanicInfo;
use core::arch::global_asm;
use crate::spinlock::SpinLock;
use core::ops::{Deref, DerefMut};
mod spinlock;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_puts("PANIC!!!");
    loop{}
}

const STACK_SIZE: usize = 4096;
#[used]
#[no_mangle]
#[link_section = ".stack"]
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

global_asm!(r#"
    .section .text._start
    .global _start
    .type _start, @function

    _start:
        #Enable floating point bits FPEN
        mrs x7, cpacr_el1
        mov x8, #(3 << 20)
        orr x7, x8, x7
        msr cpacr_el1, x7
        
        #Set up stack
        adr x7, {}
        mov sp, x7
        add sp, sp, {}
        
        bl clear_bss
        bl main
        b .
"#, sym STACK, const STACK_SIZE);

extern "C" {
    static _bss_start: u8;
    static _bss_end: u8;
}

#[no_mangle]
pub extern fn clear_bss() {
    let bss_start = unsafe {&_bss_start} as *const u8 as usize;
    let bss_end = unsafe {&_bss_end} as *const u8 as usize;
    let bss_size = bss_end - bss_start;
    assert!(bss_size%16==0);
    for x in 0..bss_size/core::mem::size_of::<u128>() {
        unsafe {
            (bss_start as *mut u128).add(x).write_volatile(0);
        }
    }
}

const SERIAL_ADDR: *mut u8 = 0x0900_0000 as *mut u8;
fn serial_putc(c: char) {
    unsafe {
        SERIAL_ADDR.write_volatile(c as u8);
    }
}

fn serial_puts(string: &str) {
    for c in string.chars() {
        serial_putc(c);
    }
}

#[no_mangle]
pub extern fn main() {
    let t = SpinLock::new(0x41_u8);
    {
        let x = t.aquire();
        let num = x.deref();
        serial_putc(*num as char);
    }
    let x = t.aquire();
    t.release();
    serial_puts("Hello world!\r\n");
    loop{}
}


#![no_main]
#![no_std]
#![feature(asm_const)]

use core::{arch::asm, panic::PanicInfo};
use core::arch::global_asm;
use crate::spinlock::SpinLock;
use crate::driver::smp::init_smp;
use crate::driver::serial::Serial;

#[macro_use]
mod print;
mod spinlock;
mod driver;
mod exception;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("Core {} panicked at {}:\r\n{}",
    get_core_id(),
    info.location().unwrap(),
    info.message());
    loop{
        unsafe {
            asm!("wfi",
            "wfe");
        }
    }
}

const NUM_CORES: usize = 4;
const STACK_SIZE: usize = 4096;
#[used]
#[no_mangle]
#[link_section = ".stack"]
static mut STACK: [u8; STACK_SIZE*NUM_CORES] = [0; STACK_SIZE*NUM_CORES];

global_asm!(include_str!("boot.s"), sym STACK, const STACK_SIZE);
global_asm!(include_str!("exception.s"));

extern "C" {
    static _bss_start: u8;
    static _bss_end: u8;
    static _start: u8;
    static _base: u8;
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

fn _get_mpid() -> u64 {
    let mpid: u64;
    unsafe {
        asm!("mrs {}, mpidr_el1", out(reg) mpid);
    }
    mpid & 0xFF
}

fn get_core_id() -> u64 {
    let tpid: u64;
    unsafe {
        asm!("mrs {}, tpidr_el1", out(reg) tpid)
    }
    tpid
}

static SERIAL_CONSOLE: SpinLock<Serial> = SpinLock::new(Serial {
    serial_addr: 0x0900_0000 as *mut u8
});

#[no_mangle]
pub extern fn main() {
    if get_core_id() == 0 {
        unsafe {
            kprint!("Hello world at {:x}!\r\n", &_base as *const u8 as usize);
            let x = &_start as *const u8 as *const fn();
            init_smp(x);
            kprintln!("SMP init successful!");
            
        }
    } else { 
        let core_id = get_core_id() as u8;
        kprintln!("Hello from core: {core_id}");
    }
    loop{}
}


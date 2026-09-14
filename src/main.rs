#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod arch;
mod drivers;
mod flib;

use crate::arch::x86_64::idt::init;
use core::panic::PanicInfo;
use x86_64::{self, instructions};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print!("something");
    init();
    instructions::interrupts::int3();
    loop {}
}

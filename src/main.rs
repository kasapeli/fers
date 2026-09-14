#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod arch;
mod drivers;
mod flib;

use crate::arch::x86_64::{gdt, idt};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("something");

    gdt::init();
    idt::init();

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    loop {}
}

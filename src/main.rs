#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod arch;
mod drivers;
mod flib;

use crate::{
    arch::x86_64::{gdt, idt},
    flib::hlt,
};
use core::panic::PanicInfo;
use x86_64;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    hlt::exec();
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("something");

    gdt::init();
    idt::init();
    unsafe { idt::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    hlt::exec();
}

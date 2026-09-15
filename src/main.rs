#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod arch;
mod drivers;
mod flib;
mod memory;
mod utils;

use crate::{
    arch::x86_64::{gdt, idt},
    flib::hlt,
    utils::fsh,
};

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::{self, VirtAddr};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    hlt::exec();
}

entry_point!(kernel);

fn kernel(boot_info: &'static BootInfo) -> ! {
    use memory::pager::BootInfoFrameAllocator;

    gdt::init();
    idt::init();

    let phy_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::pager::init(phy_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    memory::ll_alloc::init_heap(&mut mapper, &mut frame_allocator).expect("h");

    unsafe { idt::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    fsh::init();

    hlt::exec();
}

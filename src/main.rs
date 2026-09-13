#![no_std]
#![no_main]

mod drivers;

use drivers::vga::WRITER;

use core::{fmt::Write, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    WRITER.lock().write_str("Hello").unwrap();
    loop {}
}

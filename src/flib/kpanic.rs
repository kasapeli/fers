use core::panic::PanicInfo;

use crate::drivers::vga::WRITER;

pub fn panic(msg: &PanicInfo) {
    WRITER.lock().kpanic(msg);
}

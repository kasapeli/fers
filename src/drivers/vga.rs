use core::{fmt, ptr::NonNull};

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::VolatilePtr;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii: u8,
    pub color: u8,
}

pub struct VgaWriter {
    address: VolatilePtr<'static, ScreenChar>,
}

lazy_static! {
    pub static ref WRITER: Mutex<VgaWriter> = Mutex::new(VgaWriter {
        address: unsafe { VolatilePtr::new(NonNull::new_unchecked(0xB8000 as *mut ScreenChar)) },
    });
}

unsafe impl Send for VgaWriter {}

impl VgaWriter {
    pub fn write_string(&mut self, content: &str) {
        self.write_byte(content.as_bytes());
    }

    pub fn write_byte(&mut self, content: &[u8]) {
        for &byte in content.iter() {
            self.address.write(ScreenChar {
                ascii: byte,
                color: 0xf,
            });

            unsafe {
                let raw = self.address.as_raw_ptr().add(1);
                self.address = VolatilePtr::new(raw);
            }
        }
    }
}

impl core::fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

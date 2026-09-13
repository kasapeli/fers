use lazy_static::lazy_static;
use spin::Mutex;

pub struct VgaWriter {
    address: *mut u8,
    ptr: isize,
}

lazy_static! {
    pub static ref WRITER: Mutex<VgaWriter> = Mutex::new(VgaWriter {
        address: 0xB8000 as *mut u8,
        ptr: 0
    });
}

unsafe impl Send for VgaWriter {}

impl VgaWriter {
    pub fn write_string(&mut self, content: &str) {
        self.write_byte(content.as_bytes());
    }

    pub fn write_byte(&mut self, content: &[u8]) {
        for &byte in content.iter() {
            unsafe {
                *self.address.offset(self.ptr * 2) = byte;
                *self.address.offset(self.ptr * 2 + 1) = 0xf;
                self.ptr += 1;
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

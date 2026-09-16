extern crate alloc;

use core::{fmt, panic::PanicInfo, ptr::NonNull};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::VolatilePtr;

use crate::println;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii: u8,
    pub color: u8,
}

pub struct VgaWriter {
    column_position: usize,
    row_position: usize,
    base_address: NonNull<ScreenChar>,
    pub current_color: u8,
}

lazy_static! {
    pub static ref WRITER: Mutex<VgaWriter> = Mutex::new(VgaWriter {
        column_position: 0,
        row_position: 0,
        base_address: unsafe { NonNull::new_unchecked(0xB8000 as *mut ScreenChar) },
        current_color: 0xf,
    });
}

unsafe impl Send for VgaWriter {}

impl VgaWriter {
    pub fn write_string(&mut self, content: &str) {
        self.write_byte(content.as_bytes());
    }

    pub fn write_byte(&mut self, content: &[u8]) {
        for &byte in content.iter() {
            match byte {
                b'\n' => self.new_line(),

                byte => {
                    if self.column_position >= BUFFER_WIDTH {
                        self.new_line();
                    }

                    let offset = (self.row_position * BUFFER_WIDTH) + self.column_position;

                    unsafe {
                        let raw_ptr = self.base_address.as_ptr().add(offset);
                        let volatile_ptr = VolatilePtr::new(NonNull::new_unchecked(raw_ptr));
                        volatile_ptr.write(ScreenChar {
                            ascii: byte,
                            color: self.current_color,
                        });
                    }
                    self.column_position += 1;
                }
            }
        }
    }

    fn new_line(&mut self) {
        self.column_position = 0;
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            self.scroll_up();
        }
    }

    fn scroll_up(&mut self) {
        unsafe {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let source_offset = (row * BUFFER_WIDTH) + col;
                    let dest_offset = ((row - 1) * BUFFER_WIDTH) + col;

                    let source_ptr = self.base_address.as_ptr().add(source_offset);
                    let dest_ptr = self.base_address.as_ptr().add(dest_offset);

                    let val = VolatilePtr::new(NonNull::new_unchecked(source_ptr)).read();
                    VolatilePtr::new(NonNull::new_unchecked(dest_ptr)).write(val);
                }
            }

            let blank = ScreenChar {
                ascii: b' ',
                color: 0xf,
            };
            for col in 0..BUFFER_WIDTH {
                let offset = ((BUFFER_HEIGHT - 1) * BUFFER_WIDTH) + col;
                let ptr = self.base_address.as_ptr().add(offset);
                VolatilePtr::new(NonNull::new_unchecked(ptr)).write(blank);
            }
        }
    }

    pub fn clear(&mut self) {
        let blank = ScreenChar {
            ascii: b' ',
            color: self.current_color,
        };

        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let offset = (row * BUFFER_WIDTH) + col;
                unsafe {
                    let raw_ptr = self.base_address.as_ptr().add(offset);
                    let volatile_ptr = VolatilePtr::new(NonNull::new_unchecked(raw_ptr));
                    volatile_ptr.write(blank);
                }
            }

            self.column_position = 0;
            self.row_position = 0;
        }
    }

    pub fn kpanic(&mut self, msg: &PanicInfo) {
        self.current_color = 0xcf;

        let blank = ScreenChar {
            ascii: b' ',
            color: self.current_color,
        };

        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let offset = (row * BUFFER_WIDTH) + col;
                unsafe {
                    let raw_ptr = self.base_address.as_ptr().add(offset);
                    let volatile_ptr = VolatilePtr::new(NonNull::new_unchecked(raw_ptr));
                    volatile_ptr.write(blank);
                }
            }
        }

        self.column_position = 0;
        self.row_position = 0;

        let _ = core::fmt::write(self, format_args!("{}", msg));
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
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

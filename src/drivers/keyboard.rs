extern crate alloc;

use crate::print;
use alloc::string::String;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptStackFrame;

use crate::arch::x86_64::idt::{InterruptIndex, PICS};

lazy_static! {
    pub static ref INPUT: spin::Mutex<String> = spin::Mutex::new(String::new());
}

pub extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    static KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    ));

    let mut keyboard = KEYBOARD.lock();
    let mut port: Port<u8> = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    if character == '\x08' {
                        if !INPUT.lock().is_empty() {
                            INPUT.lock().push(character);
                            print!("{}", character);
                        }
                    } else {
                        INPUT.lock().push(character);
                        print!("{}", character);
                    }
                }
                DecodedKey::RawKey(_key) => {}
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

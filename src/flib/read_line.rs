extern crate alloc;

use crate::drivers::keyboard::INPUT;
use alloc::string::String;

pub fn read_string() -> String {
    loop {
        let mut input = INPUT.lock();

        while let Some(backspace_pos) = input.find('\x08') {
            if backspace_pos > 0 {
                input.remove(backspace_pos);
                input.remove(backspace_pos - 1);
            } else {
                input.remove(0);
            }
        }

        if let Some(pos) = input.find('\n') {
            let mut line = input.split_off(pos + 1);

            core::mem::swap(&mut *input, &mut line);

            if line.ends_with('\n') {
                line.pop();
            }
            if line.ends_with('\r') {
                line.pop();
            }

            return line;
        }

        drop(input);

        x86_64::instructions::hlt();
    }
}

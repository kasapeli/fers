extern crate alloc;

use crate::drivers::keyboard::INPUT;
use crate::utils::builtins::*;
use crate::{print, println};
use alloc::{str, string::String, vec::Vec};

pub fn init() -> ! {
    x86_64::instructions::interrupts::enable();
    main();
}

pub fn main() -> ! {
    print!("> ");

    let mut current_line = String::new();

    loop {
        let mut new_chars = String::new();

        x86_64::instructions::interrupts::without_interrupts(|| {
            let mut content = INPUT.lock();
            if !content.is_empty() {
                new_chars = content.clone();
                content.clear();
            }
        });

        for c in new_chars.chars() {
            match c {
                '\n' | '\r' => {
                    println!();

                    let content: Vec<&str> = current_line.trim().split_whitespace().collect();
                    parse(content);

                    current_line.clear();
                    print!("> ");
                }
                '\x08' => {
                    if !current_line.is_empty() {
                        current_line.pop();
                        print!("{}", '\x08');
                    }
                }
                _ => {
                    current_line.push(c);
                    print!("{c}");
                }
            }
        }

        x86_64::instructions::hlt();
    }
}

fn parse(content: Vec<&str>) {
    if content.is_empty() {
        return;
    }

    let cmd = content[0];
    let args = &content[1..];

    match cmd {
        "echo" => {
            echo::default(args);
        }
        "help" => {
            help::handle(&content);
        }
        "clear" => {
            clear::exec();
        }
        "ginfo" => {
            ginfo::handle(&content);
        }
        "time" => {
            time::exec(&content);
        }
        "reboot" => {
            println!("Rebooting...");
            unsafe {
                x86_64::instructions::port::Port::<u8>::new(0x64).write(0xFE);
            }
        }
        "panic" => {
            panic!("intentional panic");
        }
        _ => {
            println!("fsh: command not found: {}", cmd);
        }
    }
}

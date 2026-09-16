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

    loop {
        let mut command_line = String::new();
        let mut has_command = false; // consider replacing

        x86_64::instructions::interrupts::without_interrupts(|| {
            let mut content = INPUT.lock();

            if content.contains('\n') || content.contains('\r') {
                command_line = content.clone();
                content.clear();
                has_command = true; // consider replacing
            }
        });

        if has_command {
            let content: Vec<&str> = command_line.trim().split_whitespace().collect();

            parse(content);

            print!("> ");
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

extern crate alloc;

use crate::idt::INPUT;
use crate::{print, println};
use alloc::{str, string::String, vec::Vec};

pub fn init() -> ! {
    print!("> ");

    loop {
        let mut command_line = String::new();
        let mut has_command = false;

        x86_64::instructions::interrupts::without_interrupts(|| {
            let mut content = INPUT.lock();

            if content.contains('\n') || content.contains('\r') {
                command_line = content.clone();
                content.clear();
                has_command = true;
            }
        });

        if has_command {
            let trimmed = command_line.trim();
            let content: Vec<&str> = trimmed.split_whitespace().collect();

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
            let msg = args.join(" ");
            println!("{msg}");
        }
        "reboot" => {
            println!("Rebooting...");
            unsafe {
                x86_64::instructions::port::Port::<u8>::new(0x64).write(0xFE);
            }
        }
        _ => {
            println!("fsh: command not found: {}", cmd);
        }
    }
}

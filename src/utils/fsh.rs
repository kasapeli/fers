extern crate alloc;

use crate::drivers::keyboard::read_line;
use crate::utils::builtins::*;
use crate::{print, println};
use alloc::{str, string::String, vec::Vec};

pub fn init() -> ! {
    x86_64::instructions::interrupts::enable();
    main();
}

pub fn main() -> ! {
    loop {
        print!("fsh> ");

        let content = read_line();

        parse(content);
    }
}

fn parse(ctent: String) {
    let content: Vec<&str> = ctent.trim().split_whitespace().collect();

    if content.is_empty() {
        return;
    }

    let cmd = content[0]; // kind of a useless split tbh
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
        "poke" => {
            poke::handle(&content);
        }
        "peek" => {
            peek::handle(&content);
        }
        "shed" | "editor" => {
            shed::mloop();
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

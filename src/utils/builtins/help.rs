extern crate alloc;

use super::presets::{HelpTable, arg_err::*};
use crate::println;
use alloc::string::ToString;

pub fn handle(content: &[&str]) {
    if content.len() == 1 {
        println!("help, echo, reboot, ginfo, clear, panic");
        println!("help -v <option> for verbose help");
    } else {
        match content[1] {
            "-v" => {
                if content[2..].is_empty() {
                    println!("{}", missing(1));
                } else if content.len() >= 4 {
                    println!("{}", exceed(1));
                } else {
                    verbose_help(content[2]);
                }
            }
            _ => {
                println!("{}", invalid());
            }
        }
    }
}

pub fn verbose_help(entry: &str) {
    match entry {
        "help" => {
            let table = HelpTable {
                name: "help".to_string(),
                info: "prints help".to_string(),
                flags: alloc::vec!["-v: show verbose help".to_string()],
            };

            println!("{}", table.make());
        }
        "echo" => {
            let table = HelpTable {
                name: "echo".to_string(),
                info: "prints something to the screen".to_string(),
                flags: alloc::vec![],
            };

            println!("{}", table.make());
        }
        "reboot" => {
            let table = HelpTable {
                name: "reboot".to_string(),
                info: "reboots the system".to_string(),
                flags: alloc::vec![],
            };

            println!("{}", table.make());
        }
        "panic" => {
            let table = HelpTable {
                name: "panic".to_string(),
                info: "causes an intentional kernel panic".to_string(),
                flags: alloc::vec![],
            };

            println!("{}", table.make());
        }
        "ginfo" => {
            let table = HelpTable {
                name: "ginfo".to_string(),
                info: "fetches general info about the system".to_string(),
                flags: alloc::vec!["-s: shows specific help (options: kernel, shell)".to_string()],
            };

            println!("{}", table.make());
        }
        _ => {
            println!("{}", invalid());
        }
    }
}

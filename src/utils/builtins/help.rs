extern crate alloc;

use super::presets::{HelpTable, arg_err::*};
use crate::println;
use alloc::string::ToString;

pub fn handle(content: &[&str]) {
    if content.len() == 1 {
        println!("help, echo, reboot, ginfo, clear, panic, time, poke, peek");
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
                flags: alloc::vec![
                    "-s: shows specific help (options: kernel, shell, mem|memory|heap, cpu)"
                        .to_string()
                ],
            };

            println!("{}", table.make());
        }
        "time" => {
            let table = HelpTable {
                name: "time".to_string(),
                info: "shows the time".to_string(),
                flags: alloc::vec![
                    "-h: hour".to_string(),
                    "-m: minute".to_string(),
                    "-s: second".to_string()
                ], // TODO: get rid of .to_string() slop
            };

            println!("{}", table.make());
        }
        "poke" => {
            let table = HelpTable {
                name: "poke".to_string(),
                info: "writes a value to an address".to_string(),
                flags: alloc::vec![],
            };

            println!("{}", table.make());
        }
        "peek" => {
            let table = HelpTable {
                name: "peek".to_string(),
                info: "peeks at an address".to_string(),
                flags: alloc::vec![],
            };

            println!("{}", table.make());
        }
        _ => {
            println!("{}", invalid());
        }
    }
}

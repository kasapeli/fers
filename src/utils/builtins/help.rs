use super::presets::arg_err::*;
use crate::println;

pub fn handle(content: &[&str]) {
    if content.len() == 1 {
        println!("help, echo, reboot, ginfo, clear, panic");
        println!("help -v <option> for verbose help");
        println!("help -f to see available flags");
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
            "-f" => {
                if content.len() > 2 {
                    println!("{}", exceed(0));
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
            println!("");
        }
        _ => {
            println!("{}", invalid());
        }
    }
}

use super::presets::arg_err::*;
use crate::println;

pub fn handle(content: &[&str]) {
    if content.len() == 1 {
        println!("fers 0.1");
    } else {
        match content[1] {
            "-s" => {
                if content[2..].is_empty() {
                    println!("{}", missing(1));
                } else if content.len() >= 4 {
                    println!("{}", exceed(1));
                } else {
                    specific_info(content[2]);
                }
            }
            _ => {
                println!("{}", invalid());
            }
        }
    }
}

pub fn specific_info(entry: &str) {
    match entry {
        "kernel" => {
            println!("fers 0.1");
        }
        _ => {
            println!("{}", invalid());
        }
    }
}

use crate::println;

use super::presets::arg_err::*;

pub fn handle(content: &[&str]) {
    if content.len() == 1 {
        println!("{}", missing(1));
    } else {
        match content[1] {
            "help" => {}
            _ => println!("{}", invalid()),
        }
    }
}

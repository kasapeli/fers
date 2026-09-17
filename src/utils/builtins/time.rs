use super::presets::arg_err::invalid;
use crate::drivers::cmos::*;
use crate::println;

pub fn exec(content: &[&str]) {
    if content.len() == 1 {
        all();
    } else {
        match content[1] {
            "-s" => second(),
            "-m" => minute(),
            "-h" => hour(),
            _ => println!("{}", invalid()),
        }
    }
}

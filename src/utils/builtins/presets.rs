extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

pub mod arg_err {
    use super::*;

    pub fn exceed(expected: usize) -> String {
        format!("too many arguments provided, expected {}", expected)
    }

    pub fn missing(expected: usize) -> String {
        format!("no arguments provided, expected {}", expected)
    }

    pub fn invalid() -> String {
        String::from("invalid argument")
    }
}

pub struct HelpTable {
    pub name: String,
    pub info: String,
    pub flags: Vec<String>,
}

impl HelpTable {
    pub fn make(&self) -> String {
        let mut output = format!("command: {}\ninfo: {}\n", self.name, self.info);
        if !self.flags.is_empty() {
            output.push_str("flags:");
            for flag in &self.flags {
                output.push_str(&format!("\n{}", flag));
            }
        } else {
            output.push_str("flags: none");
        }
        output
    }
}

extern crate alloc;
use alloc::format;
use alloc::string::String;

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

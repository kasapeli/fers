use crate::{drivers::cmos::fetch_time, println, utils::builtins::presets::arg_err::exceed};

pub fn exec(content: &[&str]) {
    if content.len() > 1 {
        println!("{}", exceed(1));
    }

    let raw = content[0];

    fetch_time(raw);
}

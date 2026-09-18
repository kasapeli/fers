use super::presets::arg_err::*;
use crate::println;
use core::ptr::NonNull;
use volatile;

pub fn handle(content: &[&str]) {
    if content.len() > 2 {
        println!("{}", exceed(1));
        return;
    } else if content.len() < 2 {
        println!("{}", missing(1));
        return;
    }

    if !content[1].starts_with("0x") {
        println!("invalid address");
        return;
    }

    let addr = usize::from_str_radix(content[1].trim_start_matches("0x"), 16)
        .expect("failed to parse address");

    read(addr);
}

fn read(address: usize) {
    unsafe {
        let ptr = volatile::VolatilePtr::new(NonNull::new_unchecked(address as *mut usize));
        let res = ptr.read();
        println!("{}", res);
    }
}

use super::presets::arg_err::*;
use crate::println;
use core::ptr::NonNull;
use volatile;

pub fn handle(content: &[&str]) {
    if content.len() > 3 {
        println!("{}", exceed(2));
        return;
    } else if content.len() < 3 {
        println!("{}", missing(2));
        return;
    }

    if !content[1].starts_with("0x") {
        println!("invalid address");
        return;
    }

    let addr = usize::from_str_radix(content[1].trim_start_matches("0x"), 16)
        .expect("failed to parse address");

    let value = content[2];

    write(addr, value);
}

pub fn write<T>(address: usize, content: T)
where
    T: Copy,
{
    unsafe {
        let ptr = volatile::VolatilePtr::new(NonNull::new_unchecked(address as *mut T));
        ptr.write(content);
    }
}

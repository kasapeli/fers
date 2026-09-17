use super::presets::arg_err::*;
use crate::println;
use core::ptr::NonNull;
use volatile;

pub fn handle(content: &[&str]) {
    if content.len() == 1 {
        println!("{}", missing(2));
        return;
    } else if content.len() == 2 {
        println!("{}", missing(2));
        return;
    } else if content.len() > 3 {
        println!("{}", exceed(2));
    }

    let addr = usize::from_str_radix(content[1].trim_start_matches("0x"), 16)
        .expect("failed to parse address");
    let ptr = addr as *mut usize; // TODO: reject stuff not starting in 0x

    let value = content[2];

    unsafe {
        write(ptr, value);
    }
}

pub unsafe fn write<T>(address: *mut usize, content: T)
where
    T: Copy,
{
    unsafe {
        let ptr = volatile::VolatilePtr::new(NonNull::new_unchecked(address as *mut T));
        ptr.write(content);
    }
}

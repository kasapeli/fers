// use super::presets::arg_err::*;
// use crate::println;
// use core::ptr::NonNull;
// use volatile;

// pub fn handle(content: &[&str]) {
//     if content.is_empty() {
//         println!("{}", missing(2));
//         return;
//     } else if content.len() > 2 {
//         println!("{}", exceed(2));
//         return;
//     }

//     let address = content[0].parse::<usize>().unwrap();
//     let value = content[1];

//     unsafe {
//         write(address, value);
//     }
// }

// pub unsafe fn write<T>(address: usize, content: T)
// where
//     T: Copy,
// {
//     unsafe {
//         let ptr = volatile::VolatilePtr::new(NonNull::new_unchecked(address as *mut T));
//         ptr.write(content);
//     }
// }

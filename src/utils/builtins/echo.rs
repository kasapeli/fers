extern crate alloc;

use crate::println;

pub fn default(content: &[&str]) {
    let msg = content.join(" ");
    println!("{}", msg);
}

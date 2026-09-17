extern crate alloc;

use x86_64::instructions::port::Port;

use crate::println;

pub fn fetch_time(format: Option<&str>) {
    // consider returning u8 instead later
    match format {
        Some("s") => unsafe {
            Port::<u8>::new(0x70).write(0x00);
            let second = Port::<u8>::new(0x71).read();
            println!("{}", second);
        },
        Some("m") => unsafe {
            Port::<u8>::new(0x70).write(0x02);
            let minute = Port::<u8>::new(0x71).read();
            println!("{}", minute);
        },
        Some("h") => unsafe {
            Port::<u8>::new(0x70).write(0x04);
            let hour = Port::<u8>::new(0x71).read();
            println!("{}", hour);
        },
        _ => unsafe {
            Port::<u8>::new(0x70).write(0x00); // this is sooo messy dude
            let second = Port::<u8>::new(0x71).read();

            Port::<u8>::new(0x70).write(0x02);
            let minute = Port::<u8>::new(0x71).read();

            Port::<u8>::new(0x70).write(0x04);
            let hour = Port::<u8>::new(0x71).read();
            println!("{}:{}:{}", hour, minute, second);
        },
    }
}

// struct TimeTable {
//     second: u8,
//     minute: u8,
//     hour: u8,
// }

// impl TimeTable {
//     pub fn fetch(&mut self) -> TimeTable {
//         let second = Port::new(0x70).write(0x00);
//         let minute = Port::new(0x70).write(0x02);
//         let hour = Port::new(0x70).write(0x04);

//         let time = Port::new(0x71).read();

//         let result = Self {
//             second: time;
//         }
//     }
// }
//
//
// unused code, maybe consider using this later

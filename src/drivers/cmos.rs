extern crate alloc;

use x86_64::instructions::port::Port;

use crate::println;

fn bcd_to_int(bcd: u8) -> u8 {
    ((bcd >> 4) * 10) + (bcd & 0x0F)
}

pub fn second() {
    unsafe {
        Port::<u8>::new(0x70).write(0x00);
        let second = Port::<u8>::new(0x71).read();
        let second = bcd_to_int(second);
        println!("{}", second);
    }
}

pub fn minute() {
    unsafe {
        Port::<u8>::new(0x70).write(0x02);
        let minute = Port::<u8>::new(0x71).read();
        let minute = bcd_to_int(minute);
        println!("{}", minute);
    }
}

pub fn hour() {
    unsafe {
        Port::<u8>::new(0x70).write(0x04);
        let hour = Port::<u8>::new(0x71).read();
        let hour = bcd_to_int(hour);
        println!("{}", hour);
    }
}

pub fn all() {
    unsafe {
        Port::<u8>::new(0x70).write(0x00); // this is sooo messy dude
        let second = Port::<u8>::new(0x71).read();
        let second = bcd_to_int(second);

        Port::<u8>::new(0x70).write(0x02);
        let minute = Port::<u8>::new(0x71).read();
        let minute = bcd_to_int(minute);

        Port::<u8>::new(0x70).write(0x04);
        let hour = Port::<u8>::new(0x71).read();
        let hour = bcd_to_int(hour);

        println!("{}:{}:{}", hour, minute, second);
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

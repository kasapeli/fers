use crate::println;

pub fn handle(content: &[&str]) {
    if content.len() == 1 {
        println!("fern 0.1");
    } else {
        match content[1] {
            "-s" => {
                if content[2..].is_empty() {
                    println!("no argument provided");
                } else if content.len() >= 4 {
                    println!("too many arguments provided");
                } else {
                    specific_info(content[2]);
                }
            }
            _ => {
                println!("unknown flag");
            }
        }
    }
}

pub fn specific_info(entry: &str) {
    match entry {
        "kernel" => {
            println!("fern 0.1");
        }
        _ => {
            println!("unknown entry");
        }
    }
}

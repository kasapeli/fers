use crate::println;

pub fn handle(content: &[&str]) {
    if content.len() == 1 {
        println!("help, echo");
    } else {
        match content[1] {
            "-v" => {
                if content[2..].is_empty() {
                    println!("no argument provided");
                } else if content.len() >= 4 {
                    println!("too many arguments provided");
                } else {
                    verbose_help(content[2]);
                }
            }
            _ => {
                println!("unknown flag");
            }
        }
    }
}

pub fn verbose_help(entry: &str) {
    match entry {
        "help" => {
            println!("detailed help");
        }
        _ => {
            println!("unknown entry");
        }
    }
}

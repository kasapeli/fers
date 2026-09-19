extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use super::presets::arg_err::*;
use crate::{drivers::keyboard::read_line, print, println};

pub struct Editor {
    storage: Vec<String>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }

    pub fn push(&mut self, c: String) {
        self.storage.push(c);
    }

    pub fn view(&self) {
        if !self.storage.is_empty() {
            for (index, i) in self.storage.iter().enumerate() {
                println!("{:<2} | {}", index + 1, i);
            }
        } else {
            println!("storage is empty");
        }
    }

    pub fn view_ol(&self, i: usize) {
        if !self.storage.is_empty() {
            if i <= self.storage.len() && i > 0 {
                println!("{:<2} | {}", i, self.storage[i - 1]);
            } else {
                println!("index is out of bounds or is zero");
            }
        } else {
            println!("storage is empty");
        }
    }

    pub fn delete(&mut self) {
        if !self.storage.is_empty() {
            self.storage.pop();
        } else {
            println!("storage is empty");
        }
    }

    pub fn delete_ol(&mut self, i: usize) {
        if !self.storage.is_empty() {
            if i <= self.storage.len() && i > 0 {
                self.storage.remove(i - 1);
            } else {
                println!("index is out of bounds or is zero");
            }
        } else {
            println!("storage is empty");
        }
    }

    pub fn append(&mut self, i: usize, c: String) {
        if !self.storage.is_empty() {
            if i <= self.storage.len() && i > 0 {
                self.storage.insert(i, c);
            } else {
                println!("index is out of bounds or is zero");
            }
        } else {
            println!("storage is empty");
        }
    }

    // this means "(r)everse append" :D i dont even know if that makes any sense but its 'appending' backwards
    pub fn rappend(&mut self, i: usize, c: String) {
        if !self.storage.is_empty() {
            if i <= self.storage.len() && i > 0 {
                self.storage.insert(i - 1, c);
            } else {
                println!("index is out of bounds or is zero");
            }
        } else {
            println!("storage is empty");
        }
    }

    pub fn replace(&mut self, i: usize, c: String) {
        if !self.storage.is_empty() {
            if i <= self.storage.len() && i > 0 {
                self.storage.insert(i - 1, c);
                self.storage.remove(i);
            } else {
                println!("index is out of bounds or is zero");
            }
        } else {
            println!("storage is empty");
        }
    }
}

pub fn mloop() {
    let mut editor = Editor::new();

    loop {
        let mut input = String::new();
        let read = read_line();
        input.push_str(&read);

        match input.as_str() {
            "cmm0" => cmd(&mut editor),
            "EOF" => break,
            _ => {
                editor.push(input);
            }
        }
    }
}

pub fn cmd(editor: &mut Editor) {
    loop {
        print!("> ");
        let mut cmd = String::new();
        let read = read_line();
        cmd.push_str(&read);

        if cmd.is_empty() {
            continue;
        }

        let cmd: Vec<&str> = cmd.trim().split_whitespace().collect();

        // TODO: all of these crash if provided non-usize as an arg, check for that
        match cmd[0] {
            "v" => {
                if cmd.len() < 2 {
                    editor.view();
                } else if cmd.len() == 2 {
                    editor.view_ol(cmd[1].parse::<usize>().unwrap());
                } else {
                    println!("{}", invalid());
                }
            }
            "d" => {
                if cmd.len() < 2 {
                    editor.delete();
                } else if cmd.len() == 2 {
                    editor.delete_ol(cmd[1].parse::<usize>().unwrap());
                } else {
                    println!("{}", invalid());
                }
            }
            "a" => {
                if cmd.len() >= 3 {
                    editor.append(
                        cmd[1].parse::<usize>().unwrap(),
                        cmd[2..].join(" ").to_string(),
                    );
                } else {
                    println!("{}", invalid());
                }
            }
            "b" => {
                if cmd.len() >= 3 {
                    editor.rappend(
                        cmd[1].parse::<usize>().unwrap(),
                        cmd[2..].join(" ").to_string(),
                    );
                } else {
                    println!("{}", invalid());
                }
            }
            "r" => {
                if cmd.len() >= 3 {
                    editor.replace(
                        cmd[1].parse::<usize>().unwrap(),
                        cmd[2..].join(" ").to_string(),
                    );
                } else {
                    println!("{}", invalid());
                }
            }
            "e" | "q" => break,
            _ => {
                continue;
            }
        }
    }
}

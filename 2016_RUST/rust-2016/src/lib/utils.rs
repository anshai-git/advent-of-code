use std::{env::args, fs::File };

pub fn parse_filename() -> Option<String> {
    match args().nth(1) {
        Some(filename) => Some(filename),
        None => {
            println!("Usage: <program> <filename>");
            None
        }
    }
}

pub fn open_file(filename: &String) -> Option<File> {
    match File::open(&filename) {
        Ok(content) => Some(content),
        Err(_) => {
            eprintln!("Failed to open file {filename}");
            None
        }
    }
}

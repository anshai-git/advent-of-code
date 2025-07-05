use std::{env, fs::File, io::{BufRead, BufReader}, str::Lines};

pub fn parse_filename() -> Option<String> {
    match env::args().nth(1) {
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

pub fn get_file() -> Option<File> {
    parse_filename().and_then(|filename| open_file(&filename))
}

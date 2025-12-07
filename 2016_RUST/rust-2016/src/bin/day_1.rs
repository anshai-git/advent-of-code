use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read},
};

use rust_2016::lib::utils::{open_file, parse_filename};

enum Orientation {
    Up,
    Down,
    Left, 
    Right
}

struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(pos_x: String, pos_y: String) -> Self {
        Self {
            x: pos_x.parse::<usize>().unwrap(),
            y: pos_y.parse::<usize>().unwrap(),
        }
    }
}

fn main() {
    let content = fs::read_to_string(parse_filename().unwrap()).unwrap();
    let line = content.trim();

    let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
    let mut pos: Position = Position::new(parts[0].to_string(), parts[1].to_string());
    let rest: Vec<String> = parts[2..].iter().map(|s| s.to_string()).collect();



}

use std::{fs::File, io::{BufReader, Read}};

use aoc_2015::lib::utils::{open_file, parse_filename};

fn main() {
    let file: File = parse_filename()
        .and_then(|filename| open_file(&filename))
        .expect("Failed to open file.");

    let mut position: i32 = 0;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1];
    let mut step = 0;

    while let Ok(c) = reader.read(&mut buffer) {
        if c == 0 {
            break;
        }

        step += 1;
        match buffer[0] as char {
            '(' => {
                position += 1;
                println!("[{}]: '(' {}", step, position);
            } ,
            ')' => {
                position -= 1;
                println!("[{}]: ')' {}", step, position);
            } ,
            _ => {
                println!("Invalid character");
            }
        }
    }

    println!("Position: {}", position);
}

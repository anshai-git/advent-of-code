use std::{env, fs::read_to_string, process};

const DIGIT_MAP: [(&str, i32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    let program: String = env::args().next().unwrap();
    if let Some(file_path) = env::args().nth(1) {
        process_file(&file_path);
    } else {
        eprintln!("Invalid file path.\nUsage: {program} <file path>\n");
        process::exit(1);
    }
}

fn process_file(path: &String) -> () {
    if let Ok(raw) = read_to_string(path) {
        let mut sum = 0;
        for line in parse_lines(raw) {
            let first = line.first().unwrap_or(&(0 as u32));
            let last = line.last().unwrap_or(&(0 as u32));
            sum += first * 10 + last;
        }

        println!("SUM: {sum}");
    } else {
        eprintln!("Failed to read file");
        process::exit(1);
    }
}

fn parse_lines(raw_str: String) -> Vec<Vec<u32>> {
    raw_str.lines().map(|l| parse_line(l)).collect()
}

fn parse_line(line: &str) -> Vec<u32> {
    let mut digits = Vec::new();
    for i in 0..line.chars().count() {
        if let Some(d) = parse_digit(line.chars().nth(i).unwrap(), line, i) {
            digits.push(d);
        }
    }
    digits
}

fn parse_digit(c: char, str: &str, index: usize) -> Option<u32> {
    if c.is_digit(10) {
        return Some(c.to_digit(10).unwrap());
    }

    if c.is_alphabetic() {
        for ele in DIGIT_MAP {
            if str[index..].starts_with(ele.0) {
                return Some(ele.1 as u32);
            }
        }
    }

    None
}

use std::{env, fs::read_to_string, process };

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
    raw_str.lines()
        .map(|l| parse_line(l))
        .collect()
}

fn parse_line(line: &str) -> Vec<u32> {
        line.chars()
            .filter_map(|c| is_digit(c))
            .collect()
}

fn is_digit(c: char) -> Option<u32> {
    if c > '0' && c <= '9' {
        return Some(c.to_digit(10).unwrap());
    }
    None
}

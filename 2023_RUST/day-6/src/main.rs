use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn print_usage(program: &str) -> () {
    println!("Usage: {program} <filename>");
}

fn parse_numbers(line: String) -> Vec<u32> {
    line.split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|e| !e.is_empty())
        .map(|e| e.trim())
        .map(|e| e.parse::<u32>())
        .flatten()
        .collect()
}

fn main() {
    let program: String = args().next().unwrap();
    let filename: String = match args().count() {
        2 => args().nth(1).unwrap(),
        _ => {
            eprintln!("\nInvalid number of arguments.");
            print_usage(&program);
            process::exit(1)
        }
    };

    let file: File = match File::open(&filename) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Failed to read file {filename}");
            process::exit(1)
        }
    };

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .flatten()
        .map(String::from)
        .collect();

    let times: Vec<u32> = parse_numbers(lines.get(0).unwrap().to_string());
    let distances: Vec<u32> = parse_numbers(lines.get(1).unwrap().to_string());

    let mut result = 1;
    for (&time, distance) in times.iter().zip(distances) {
        let mut count = 0;
        for i in 0..time {
            if distance < (time - i) * i {
                count += 1;
            }
        }
        result *= count;
    }

    println!("RESULT: {result}");
}

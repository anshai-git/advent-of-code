use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn print_usage(program: &str) -> () {
    println!("Usage: {program} <filename>");
}

fn parse_number(line: String) -> u64 {
    line.split(':')
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap()
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

    let time: u64 = parse_number(lines.get(0).unwrap().to_string());
    let distance: u64 = parse_number(lines.get(1).unwrap().to_string());

    let mut result = 0;
    for i in 0..time {
        if distance < (time - i) * i {
            result += 1;
        }
    }

    println!("RESULT: {result}");
}

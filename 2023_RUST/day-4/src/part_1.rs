use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn print_usage(program: &str) -> () {
    eprintln!("Usage: {program} <filename>");
}

fn parse_numbers(part: String) -> Vec<u64> {
    part.split(' ')
        .filter(|s| !s.is_empty())
        .map(|n| n.trim())
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn main() {
    let program: String = env::args().nth(0).unwrap();
    let filename: String = match env::args().count() {
        2 => env::args().nth(1).unwrap(),
        _ => {
            eprintln!("Invalid number of arguments");
            print_usage(&program);
            process::exit(1)
        }
    };

    let file = match File::open(&filename) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read file {filename}");
            process::exit(1)
        }
    };

    let mut sum: u64 = 0;
    for line in BufReader::new(file).lines() {
        let mut parts: Vec<String> = line.unwrap()
            .split(':').nth(1).unwrap()
            .split('|') .map(|p| p.trim())
            .map(String::from)
            .collect();

        let own_numbers: Vec<u64> = parse_numbers(parts.pop().unwrap());
        let winning_numbers: Vec<u64> = parse_numbers(parts.pop().unwrap());

        let mut points = 0;
        for num in own_numbers {
            if winning_numbers.contains(&num) {
                points = if points == 0 {1} else {2 * points};
            }
        }
        sum += points;
    }

    println!("SUM {sum}");
}

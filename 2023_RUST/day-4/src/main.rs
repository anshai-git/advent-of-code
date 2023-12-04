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
    let mut instances_by_index: [u64; 1024] = [1; 1024];
    for (c_index, card) in BufReader::new(file).lines().enumerate() {
        let mut matches: u64 = 0;
        let mut parts: Vec<String> = card.unwrap()
            .split(':').nth(1).unwrap()
            .split('|') .map(|p| p.trim())
            .map(String::from)
            .collect();

        let own_numbers: Vec<u64> = parse_numbers(parts.pop().unwrap());
        let winning_numbers: Vec<u64> = parse_numbers(parts.pop().unwrap());

        for num in own_numbers {
            if winning_numbers.contains(&num) {
                matches += 1;
            }
        }

        for i in c_index + 1..=(c_index + matches as usize) {
            instances_by_index[i] = instances_by_index[i] + instances_by_index[c_index];
        }

        sum += instances_by_index.get(c_index).unwrap();
    }

    println!("SUM {sum}");
}

use std::{collections::HashMap, env::args, fs::File, io::{BufRead, BufReader}, process::exit};

fn main() {
    let program: String = match args().next() {
        Some(name) => name,
        None => {
            eprintln!("Program name missing.");
            exit(1);
        }
    };

    let filename: String = match args().nth(1) {
        Some(filename) => filename,
        None => {
            println!("Usage: {program} <filename>");
            exit(1);
        }
    };

    let file: File = match File::open(&filename) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to open file {filename}");
            exit(1);
        }
    };

    let reader: BufReader<File> = BufReader::new(file);

    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    for line in reader.lines().flatten() {
        if let Some(l) = line.split("   ").nth(0).and_then(|it| it.parse::<i64>().ok()) {
            let position = left.iter().position(|&x| x > l).unwrap_or(left.len());
            left.insert(position, l);
        };

        if let Some(r) = line.split("   ").nth(1).and_then(|it| it.parse::<i64>().ok()) {
            let position = right.iter().position(|&x| x > r).unwrap_or(right.len());
            right.insert(position, r);
        }
    }

    let mut counts: HashMap<i64, usize> = HashMap::new();
    for &num in &right {
        *counts.entry(num).or_insert(0) += 1;
    }
    let sum: usize = left.iter()
        .map(|l| *l as usize * counts.get(l).unwrap_or(&0))
        .sum();

    println!("{sum}");
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
    num, u128,
};

extern crate aoc_2025;
use aoc_2025::lib::utils::{open_file, parse_filename};

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .expect("Failed to open file");

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .flatten()
        .map(|l| l.to_string())
        .collect();

    let mut numbers: Vec<Vec<u128>> = Vec::new();

    for line in lines.iter().take(lines.len() - 1) {
        let parsed: Vec<u128> = line
            .split_whitespace()
            .into_iter()
            .map(|it| it.parse::<u128>())
            .flatten()
            .collect();

        numbers.push(parsed);
    }

    let operations: Vec<String> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|it| it.to_string())
        .collect();

    for n in &numbers {
        println!("{:?}", n);
    }

    let mut sum: u128 = 0;

    for (i, operation) in operations.iter().enumerate() {
        match operation.as_str() {
            "+" => {
                let mut x: u128 = 0;
                for n in &numbers {
                    x += n[i];
                    println!("[{}] OP {:?} N: {:?}", i, operation, n[i]);
                }
                println!("Result: {}\n", x);
                sum += x;
            }
            "*" => {
                let mut x: u128 = 1;
                for n in &numbers {
                    x *= n[i];
                    println!("[{}] OP {:?} N: {:?}", i, operation, n[i]);
                }
                println!("Result: {}\n", x);
                sum += x;
            }
            _ => {
                println!("unknown operation");
            }
        }
    }

    println!("SUM: {}", sum);
}

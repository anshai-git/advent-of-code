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

    let chars: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .flatten()
        .map(|l| l.to_string().chars().collect())
        .collect();

    println!("{:?}", chars);

    let mut groups: Vec<(Vec<u128>, char)> = Vec::new();

    let mut numbers: Vec<u128> = Vec::new();
    for col in (0..chars[0].len()).rev() {
        let mut num: u128 = 0;

        for row in 0..chars.len() {
            println!("[{}][{}] {}", row, col, chars[row][col]);

            if chars[row][col].is_numeric() {
                num *= 10;
                num += chars[row][col].to_string().parse::<u128>().unwrap();
            }

            if chars[row][col] == '+' || chars[row][col] == '*' {
                numbers.push(num);
                num = 0;
                groups.push((numbers, chars[row][col]));
                numbers = Vec::new();
            }
        }

        if num != 0 {
            numbers.push(num);
        }
    }

    let mut sum: u128 = 0;
    for g in groups {
        println!("Group: {:?}", g);

        if g.1 == '+' {
            let mut group_sum: u128 = 0;
            for n in g.0 {
                group_sum += n;
            }
            println!("Group sum {}", group_sum);
            sum += group_sum;
        } else if g.1 == '*' {
            let mut group_sum: u128 = 1;
            for n in g.0 {
                group_sum *= n;
            }
            println!("Group sum {}", group_sum);
            sum += group_sum;
        }
    }

    println!("{:?}", sum);
}

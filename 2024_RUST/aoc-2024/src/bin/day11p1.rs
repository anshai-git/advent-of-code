use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use aoc_2024::lib::util::{open_file, parse_filename};

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .unwrap();

    let mut stones: Vec<u64> = BufReader::new(&file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|it| it.parse::<u64>().ok())
        .collect();

    for blink_index in 1..26 {
        let mut new_stones: Vec<u64> = Vec::new();
        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
            } else if has_even_number_of_digits(&stone) {
                let mid = stone.to_string().chars().count() / 2;
                let stone_string = stone.to_string();
                new_stones.push(stone_string[..mid].parse::<u64>().unwrap());
                new_stones.push(stone_string[mid..].parse::<u64>().unwrap());
            } else {
                new_stones.push(2024 * stone);
            }
        }
        stones = new_stones;
        println!("{:<3?} :: ({})", blink_index, stones.len());
    }
}

fn has_even_number_of_digits(n: &u64) -> bool {
    n.to_string().chars().count() % 2 == 0
}

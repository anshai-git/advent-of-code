use std::{
    fs::File,
    io::{BufRead, BufReader},
};

extern crate aoc_2025;
use aoc_2025::lib::utils::{open_file, parse_filename};

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .expect("Failed to open file");

    let mut grid: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .flatten()
        .map(|l| l.chars().collect())
        .collect();

    for row in &grid {
        println!("{:?}", row);
    }

    let start: (usize, usize) = {
        let mut val = (0, 0);
        for i in 0..grid[0].len() {
            if grid[0][i] == 'S' {
                val = (0, i);
                break;
            }
        }
        val
    };
}

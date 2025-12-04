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

    let grid: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .flatten()
        .into_iter()
        .map(|l| l.chars().collect())
        .collect();

    let mut count: usize = 0;
    for pos_y in 0..grid.len() {
        for pos_x in 0..grid[0].len() {
            if grid[pos_y][pos_x] == '@' && count_adjacent(&grid, pos_x, pos_y) <= 4 {
                println!("Accessible: {}-{}", pos_y, pos_x);
                count += 1;
            }
        }
    }
    println!("Count: {}", count);
}

pub fn count_adjacent(grid: &Vec<Vec<char>>, pos_x: usize, pos_y: usize) -> usize {
    let start_x = if pos_x == 0 { 0 } else { pos_x - 1 };
    let end_x = if pos_x == grid[0].len() - 1 {
        grid[0].len() - 1
    } else {
        pos_x + 1
    };
    let start_y = if pos_y == 0 { 0 } else { pos_y - 1 };
    let end_y = if pos_y == grid.len() - 1 {
        grid.len() - 1
    } else {
        pos_y + 1
    };

    let mut count: usize = 0;
    for i in start_y..=end_y {
        for j in start_x..=end_x {
            if grid[i][j] == '@' {
                count += 1;
            }
        }
    }

    count
}

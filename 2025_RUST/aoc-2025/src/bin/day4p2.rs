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
        .into_iter()
        .map(|l| l.chars().collect())
        .collect();

    let mut count: usize = 0;
    while let Some(accessible_positions) = find_accessible(&grid) {
        println!("Accessible Count: {}", accessible_positions.len());
        count += accessible_positions.len();
        for pos in accessible_positions {
            grid[pos.0][pos.1] = '.';
        }
    }
    println!("Count: {}", count);
}

pub fn find_accessible(grid: &Vec<Vec<char>>) -> Option<Vec<(usize, usize)>> {
    let mut accessible: Vec<(usize, usize)> = Vec::new();
    for pos_y in 0..grid.len() {
        for pos_x in 0..grid[0].len() {
            if grid[pos_y][pos_x] == '@' && count_adjacent(&grid, pos_x, pos_y) <= 4 {
                accessible.push((pos_y, pos_x));
            }
        }
    }
    if accessible.is_empty() {
        None
    } else {
        Some(accessible)
    }
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

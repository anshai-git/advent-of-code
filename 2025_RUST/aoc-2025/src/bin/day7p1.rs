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

    grid[start.0][start.1] = '|';

    for row in &grid {
        println!("{:?}", row);
    }

    let mut count: usize = 0;
    for i in 0..grid.len() - 1 {
        for j in 0..grid[i].len() {
            if grid[i][j] == '|' {
                println!("[{}][{}] FOUND |", i, j);
                if grid[i + 1][j] == '.' {
                    println!("[{}][{}] setting |", i + 1, j);
                    grid[i + 1][j] = '|';
                } else if grid[i + 1][j] == '^' {
                    count += 1;
                    grid[i + 1][j - 1] = '|';
                    grid[i + 1][j + 1] = '|';
                }
            }
        }

        for row in &grid {
            println!("{:?}", row);
        }
        println!("=========================================================================\n");
    }

    println!("Start: {:?}", start);
    println!("Count: {:?}", count);
}

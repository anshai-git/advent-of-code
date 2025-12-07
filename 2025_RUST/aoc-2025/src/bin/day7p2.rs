use std::{
    collections::HashMap,
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
        .map(|l| l.chars().collect())
        .collect();
    let start = (0, grid[0].iter().position(|&c| c == 'S').unwrap());

    let mut acc: usize = 0;
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    traverse(&grid, start.1, start.0, &mut acc, &mut cache);

    println!("Acc: {}", acc);
}

fn traverse(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    acc: &mut usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> () {
    if let Some(cached_acc) = cache.get(&(x, y)) {
        *acc += cached_acc;
        return;
    }

    if y + 1 == grid.len() {
        *acc += 1;
        return;
    }

    if grid[y + 1][x] == '.' {
        traverse(grid, x, y + 1, acc, cache);
    }

    if grid[y + 1][x] == '^' {
        let current_acc = *acc;
        traverse(grid, x - 1, y + 1, acc, cache);
        traverse(grid, x + 1, y + 1, acc, cache);
        cache.insert((x, y), *acc - current_acc);
    }
}

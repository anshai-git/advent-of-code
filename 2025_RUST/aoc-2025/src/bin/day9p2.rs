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

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .flatten()
        .map(|l| l.to_string())
        .collect();

    let mut red_tiles: Vec<(usize, usize)> = lines
        .iter()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect();
    red_tiles.push(red_tiles[0]);

    let size_x: usize = red_tiles.iter().map(|t| t.0).max().unwrap() + 3;
    let size_y: usize = red_tiles.iter().map(|t| t.1).max().unwrap() + 2;

    println!("MAX X: {} | MAX Y {}", size_x, size_y);

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; size_x]; size_y];
    print_grid(&grid);

    for rt in &red_tiles {
        grid[rt.1][rt.0] = '#';
    }
    print_grid(&grid);

    for i in 0..(red_tiles.len() - 1) {
        let start_tile = red_tiles[i];
        let end_tile = red_tiles[i + 1];

        if start_tile.0 == end_tile.0 {
            let s = start_tile.1.min(end_tile.1);
            let e = start_tile.1.max(end_tile.1);
            for y in (s+1)..e {
                grid[y][start_tile.0] = 'X';
            }
        }

        if start_tile.1 == end_tile.1 {
            let s = start_tile.0.min(end_tile.0);
            let e = start_tile.0.max(end_tile.0);
            for x in (s+1)..e {
                grid[start_tile.1][x] = 'X';
            }
        }
    }

    print_grid(&grid);

    for y in 0..grid.len() {
        let mut x_count = 0;
        let mut s_count = 0;
        for x in 0..grid[0].len() {
            if grid[y][x] == 'X' {
                x_count += 1;
                continue;
            }

            if grid[y][x] == '#' {
                s_count += 1;
                continue;
            }

            if x_count % 2 == 1 || s_count % 2 == 1 {
                grid[y][x] = 'X';
            }
        }
    }
    print_grid(&grid);

}

fn print_grid(grid: &Vec<Vec<char>>) {
    println!("---------------------------");
    for row in grid {
        for col in row {
            print!("{} ", col);
        }
        println!("");
    }
}

use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::{BufRead, BufReader, Read},
    usize,
};

use aoc_2015::lib::utils::{open_file, parse_filename};

fn main() {
    let file: String = parse_filename()
        .and_then(|filename| fs::read_to_string(&filename).ok())
        .expect("Failed to read file");

    let mut position: (isize, isize) = (0, 0);

    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    visited.insert(position);

    for step in file.chars() {
        println!("{:?}, {:?}, {:?}", step, position, visited);
        match step {
            '>' => position.0 += 1,
            '<' => position.0 -= 1,
            '^' => position.1 += 1,
            'v' => position.1 -= 1,
            _ => {
                println!("Invalid character");
            }
        }
        visited.insert(position);
    }

    println!("Distinct houses: {}", visited.len());
}

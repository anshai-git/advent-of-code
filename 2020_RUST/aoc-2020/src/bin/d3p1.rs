use std::{char, fs::File, io::{BufRead, BufReader}, process};

use aoc_2020::lib::util::get_file;

pub enum Cell {
    Empty,
    Tree
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
       match value {
           '.' => Cell::Empty,
           '#' => Cell::Tree,
           _ => {
               eprintln!("Invalid character.");
               process::exit(1);
           }
       }
    }
}

fn main() {
    let file: File = get_file().unwrap();

    let map: Vec<Vec<Cell>> = BufReader::new(&file).lines()
        .flatten()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect();

    let mut x = 0;
    let mut y = 0;

    let step_x = 1;
    let step_y = 3;

    let mut count = 0;
    while x < map.len() {
        if let Cell::Tree = map[x][y] {
            count += 1;
        }

        x += step_x;
        y = (y + step_y) % map[0].len();
    }

    println!("COUNT: {:?}", count);
}
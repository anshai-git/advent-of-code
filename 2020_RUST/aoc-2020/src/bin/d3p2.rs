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

    let mut slopes: Vec<(usize, usize)> = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut positions: Vec<(usize, usize)> = vec![(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
    let mut counts: Vec<usize> = vec![0, 0, 0, 0, 0];

    while !slopes.is_empty() {
        for (i, slope) in slopes.iter().enumerate() {
            let x = positions[i].0 + slope.0;
            let y = (positions[i].1 + slope.1) % map[0].len();

            if x >= map.len() {
                slopes.remove(i);
                break;
            }

            if let Cell::Tree = map[x][y] {
                counts[i] += 1;
            }

            positions[i].0 = x;
            positions[i].1 = y;
        }
    }

    println!("Counts: {:?}", counts);
    let mut product = 1;
    for el in counts {
        product *= el;
    }
    println!("Product: {:?}", product);

}
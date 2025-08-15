use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::{BufRead, BufReader},
};

use aoc_2015::lib::utils::{open_file, parse_filename};

struct Ztring {
    pub chars: Vec<char>,
}

impl Ztring {
    pub fn new(value: String) -> Self {
        Self {
            chars: value.chars().collect(),
        }
    }

    pub fn is_nice(&self) -> bool {
        let mut repeats: bool = false;
        let mut count_by_char: HashMap<&char, usize> = HashMap::new();
        for i in 0..self.chars.len() {
            if i + 2 <= self.chars.len()
                && self.chars.get(i).unwrap() == self.chars.get(i + 2).unwrap()
            {
                repeats = true;
            }
            count_by_char
                .entry(self.chars.get(i).unwrap())
                .and_modify(|v| *v += 1)
                .or_insert_with(|| 1);
        }

        true
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|filename| open_file(&filename))
        .expect("Failed to read file");

    let ztrings: Vec<Ztring> = BufReader::new(&file)
        .lines()
        .flatten()
        .map(|l| Ztring::new(l))
        .collect();

    let count = ztrings
        .iter()
        .filter(|z| {
            let is_nice = z.is_nice();
            println!("{:?} :: {}", z.chars, is_nice);
            is_nice
        })
        .count();

    println!("COUNT: {}", count);
}

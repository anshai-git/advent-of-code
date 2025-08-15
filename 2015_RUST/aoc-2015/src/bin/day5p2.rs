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
        let mut has_double: bool = false;
        let mut count_by_pair: HashMap<(&char, &char), usize> = HashMap::new();
        for i in 0..self.chars.len() {
            if i + 2 < self.chars.len()
                && self.chars.get(i).unwrap() == self.chars.get(i + 2).unwrap()
            {
                repeats = true;
            }
            if i + 1 < self.chars.len() {
                if let Some(pos) =
                    count_by_pair.get(&(self.chars.get(i).unwrap(), (self.chars.get(i + 1).unwrap())))
                {
                    if i - pos > 1 {
                        has_double = true;
                    }
                }

                count_by_pair.insert((self.chars.get(i).unwrap(), (self.chars.get(i + 1).unwrap())), i);
            }

            if repeats && has_double {
                return true;
            }
        }

        false
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

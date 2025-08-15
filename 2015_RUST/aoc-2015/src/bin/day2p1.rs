use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

use aoc_2015::lib::utils::{open_file, parse_filename};

struct Present {
    pub length: usize,
    pub width: usize,
    pub height: usize,
}

impl From<String> for Present {
    fn from(value: String) -> Self {
        let [l, w, h]: [usize; 3] = value
            .split('x')
            .map(|p| p.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Present { length: l, width: w, height: h }
    }
}

impl Present {
    fn paper(&self) -> usize {
        (2 * self.length * self.width) + (2 * self.width * self.height) + (2 * self.height * self.length) + (self.length * self.width).min(self.width * self.height).min(self.height * self.length)
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|filename| open_file(&filename))
        .expect("Failed to open file.");

    let presents: Vec<Present> = BufReader::new(file)
        .lines()
        .into_iter()
        .flatten()
        .map(|l| Present::from(l))
        .collect();

    let paper: usize = presents.iter().map(|p| p.paper()).sum();

    println!("PAPER {}", paper);
}

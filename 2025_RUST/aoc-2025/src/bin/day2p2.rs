use std::{
    fs::File,
    io::{BufRead, BufReader},
};

extern crate aoc_2025;
use aoc_2025::lib::utils::{open_file, parse_filename};

#[derive(Debug)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

impl Range {
    pub fn sum_invalid(&self) -> u64 {
        let mut sum: u64 = 0;

        'a: for i in self.start..=self.end {
            let num_str = i.to_string();
            for j in 1..num_str.chars().count() {
                if num_str.chars().count() % j == 0 {
                    let chars: Vec<char> = num_str.chars().collect();
                    let chunks: Vec<&[char]> = chars.chunks(j).collect();
                    if chunks.iter().all(|c| *c == chunks[0]) {
                        println!("Chunks: {:?}", chunks);
                        sum += i;
                        continue 'a;
                    }
                }

            }
        }

        sum
    }
}

impl From<String> for Range {
    fn from(value: String) -> Self {
        let (start, end): (&str, &str) = value.split_once("-").unwrap();
        let (start, end): (u64, u64) = (start.parse().unwrap(), end.parse().unwrap());

        Self { start, end }
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .unwrap();

    let ranges: Vec<Range> = BufReader::new(file)
        .lines()
        .nth(0)
        .unwrap()
        .unwrap()
        .to_string()
        .split(",")
        .into_iter()
        .map(|range_str| Range::from(range_str.to_string()))
        .collect();

    let mut sum: u64 = 0;
    for range in ranges {
        println!("Range: {:?} :: {}", range, range.sum_invalid());
        sum += range.sum_invalid();
    }

    println!("Sum: {}", sum);
}

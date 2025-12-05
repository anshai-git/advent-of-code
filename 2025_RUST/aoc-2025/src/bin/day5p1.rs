use std::{
    fs::File,
    io::{BufRead, BufReader},
};

extern crate aoc_2025;
use aoc_2025::lib::utils::{open_file, parse_filename};

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    pub fn from(value: &String) -> Self {
        let (start, end): (&str, &str) = value.split_once("-").unwrap();
        Self {
            start: start.parse::<usize>().unwrap(),
            end: end.parse::<usize>().unwrap(),
        }
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .expect("Failed to open file");

    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();

    let values: Vec<Vec<String>> = lines.split(|l| l.is_empty()).map(|g| g.to_vec()).collect();

    let ranges: Vec<Range> = values
        .get(0)
        .unwrap()
        .iter()
        .map(|l| Range::from(l))
        .collect();

    let values: Vec<usize> = values
        .get(1)
        .unwrap()
        .iter()
        .map(|v| v.parse::<usize>())
        .flatten()
        .collect();

    let mut count: usize = 0;
    for v in values {
        if ranges_contain(&ranges, v) {
            count += 1;
        }
    }

    println!("COUNT: {}", count);
}

fn ranges_contain(ranges: &Vec<Range>, value: usize) -> bool {
    for range in ranges {
        if value >= range.start && value <= range.end {
            println!("{:?} IN RANGE: {:?} ", value, range);
            return true;
        }
    }

    false
}

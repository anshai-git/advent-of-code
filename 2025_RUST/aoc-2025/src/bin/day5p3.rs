use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

extern crate aoc_2025;
extern crate core;
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

    pub fn count(&self) -> usize {
        if self.start == 0 && self.end == 0 {
            return 0;
        }

        if self.start > self.end {
            println!("error :: {} {}", self.start, self.end);
            panic!("error");
        }

        if self.start == self.end {
            1
        } else {
            (self.end - self.start) + 1
        }
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .expect("Failed to open file");

    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();
    let values: Vec<Vec<String>> = lines.split(|l| l.is_empty()).map(|g| g.to_vec()).collect();

    let mut ranges: Vec<Range> = values
        .get(0)
        .unwrap()
        .iter()
        .map(|l| Range::from(l))
        .collect();

    ranges.sort_by_key(|r| r.start);

    for i in 0..ranges.len() {
        let new_range = align_range(i, &ranges);
        ranges[i] = new_range;
    }

    let count: usize = ranges.into_iter().map(|r| r.count()).sum();
    println!("Count: {}", count);
}

fn align_range(range_index: usize, ranges: &Vec<Range>) -> Range {
    let mut range: Range = Range {
        start: ranges[range_index].start,
        end: ranges[range_index].end,
    };

    for i in range_index..ranges.len() {
        let r = ranges.get(i).unwrap();

        if range.start >= r.start && range.end <= r.end {
            range.start = 0;
            range.end = 0;
            break;
        }

        if range.end >= r.start && range.end < r.end {
            println!("end of {:?} in {:?}", range, r);
            range.end = r.start - 1;
            println!("with new end {:?}", range);
            break;
        }
    }

    range
}

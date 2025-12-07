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

    println!("\n======================\n");
    for range in &ranges {
        println!("{:?}", range);
    }
    println!("\n======================\n");

    ranges.sort_by_key(|r| r.start);

    println!("\n======================\n");
    for range in &ranges {
        println!("{:?}", range);
    }
    println!("\n======================\n");

    for i in 0..ranges.len() {
        while let Some(range) = align_range(i, &ranges) {
            println!("[{}] :: {:?}", i, range);
            ranges[i] = range;
        }
    }

    for i in 0..ranges.len() {
        while let Some(range) = align_range(i, &ranges) {
            println!("[{}] :: {:?}", i, range);
            ranges[i] = range;
        }
    }

    println!("\n======================\n");
    for range in &ranges {
        println!("{:?}", range);
    }
    println!("\n======================\n");
    let count: usize = ranges.into_iter().map(|r| r.count()).sum();
    println!("Count: {}", count);
}

fn align_range(range_index: usize, ranges: &Vec<Range>) -> Option<Range> {
    let mut range: Range = Range {
        start: ranges[range_index].start,
        end: ranges[range_index].end,
    };

    if range.start == 0 && range.end == 0 {
        return None;
    }

    for i in 0..ranges.len() {
        let r = ranges.get(i).unwrap();

        if range_index == i || (r.start == 0 && r.end == 0) {
            continue;
        }

        println!("Comparing {:?} with {:?} :: result {}", range, r, range.start >= r.start && range.end <= r.end);
        if range.start >= r.start && range.end <= r.end {
            range.start = 0;
            range.end = 0;
            return Some(range);
        }

        if range.end >= r.start && range.end <= r.end {
            println!("end of {:?} in {:?}", range, r);
            range.end = r.start - 1;
            println!("with new end {:?}", range);
            return Some(range);
        }
    }

    None

}

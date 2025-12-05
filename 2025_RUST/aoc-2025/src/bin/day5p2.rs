use core::panic;
use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

extern crate core;
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

    pub fn count(&self) -> usize {
        if self.start == 0 && self.end == 0 {
            return 0;
        }

        if self.start > self.end {
            println!("error :: {} {}", self.start, self.end);
            panic!("error");
        }

        if self.start == self.end {
            // println!("EQUAL :: {:?}", self);
            1
        } else {
            // println!("{} - {}", self.start, self.end);
            (self.end - self.start) + 1
        }
    }

    pub fn print(&self) -> () {
        for i in self.start..=self.end {
            print!("{}, ", i);
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

    let mut count: usize = 0;

    println!("PRINT \n\n");
    for range in ranges {
        // range.print();
        count += range.count();
    }

    println!("Count: {}", count);
}

fn align_range(range_index: usize, ranges: &Vec<Range>) -> Range {
    let mut range: Range = Range {
        start: ranges[range_index].start,
        end: ranges[range_index].end,
    };


    for i in 0..ranges.len() {
        if i == range_index {
            continue;
        }

        let r = ranges.get(i).unwrap();

        if range.start >= r.start && range.end <= r.end {
            println!("range {:?} is included in {:?}", range, r);

            range.start = 0;
            range.end = 0;
            return range;
        }

        if range.end > r.start && range.end < r.end {
            println!("end of {:?} in {:?}", range, r);
            range.end = r.start - 1;
            println!("with new end {:?}", range);
        }

        if range.start < r.end && range.start > r.start {
            println!("start of {:?} in {:?}", range, r);
            range.start = r.end + 1;
            println!("with new start {:?}", range);
        }
    }

    range
}

// 11 12 13 14 15 16 17 18 19 20 21
//                --------------
//    --------------------
//    ----------------


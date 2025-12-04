use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
};

extern crate aoc_2025;
use aoc_2025::lib::utils::{open_file, parse_filename};

#[derive(Debug)]
struct Bank {
    pub batteries: Vec<usize>,
}

impl Bank {
    pub fn largest_joltage(&self, batteries_count: usize) -> usize {
        let mut batteries_found: Vec<(usize, usize)> = Vec::new();
        for _ in 0..batteries_count {
            let max_index = &batteries_found
                .iter()
                .map(|b| b.0)
                .max()
                .or_else(|| Some(0 as usize))
                .unwrap();

            if let Some(largest_joltage_battery) = find_largest(
                &self.batteries,
                if max_index < &(self.batteries.len() - 1) && !batteries_found.is_empty() {
                    max_index + 1
                } else {
                    0
                },
                batteries_found.iter().map(|b| b.0).collect(),
            ) {
                println!("Find for bank: {:?} :: {:?}", self, largest_joltage_battery);
                batteries_found.push(largest_joltage_battery);
            }
        }
        batteries_found.sort_by_key(|item| item.0);

        let mut jolts = 0;
        for battery in batteries_found {
            jolts *= 10;
            jolts += battery.1;
        }

        jolts
    }
}

impl From<String> for Bank {
    fn from(value: String) -> Self {
        Self {
            batteries: value
                .chars()
                .into_iter()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect(),
        }
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .expect("Failed to open file");

    let banks: Vec<Bank> = BufReader::new(file)
        .lines()
        .map(|l| Bank::from(l.unwrap().to_string()))
        .collect();

    let mut sum = 0;
    for bank in banks {
        let jolts = bank.largest_joltage(2);
        println!("Bank {:?} :: Joltage {:?}", bank, jolts);
        sum += jolts;
    }

    println!("SUM: {}", sum);
    
}

pub fn find_largest(
    nums: &Vec<usize>,
    start_index: usize,
    ignore_indexes: Vec<usize>,
) -> Option<(usize, usize)> {
    if nums.is_empty() {
        return None;
    }

    if nums.len() == 1 {
        return Some((0, nums[0]));
    }

    let mut largest: (usize, usize) = (start_index, nums[start_index]);
    for i in (start_index)..nums.len() {
        if ignore_indexes.contains(&i) {
            continue;
        }

        if let Some(n) = nums.get(i) {
            if n > &largest.1 {
                largest = (i, *n);
            }
        }
    }

    Some(largest)
}

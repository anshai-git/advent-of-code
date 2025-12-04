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
        let mut batteries = self.batteries.clone();

        while let Some(weak) = find_low_from_left(&batteries) {
            if batteries.len() == batteries_count {
                break;
            }

            batteries.remove(weak.0);
            println!("Removed from left: {:?} :: {:?}", weak, batteries);
        }

        if batteries.len() > batteries_count {
            while let Some(same) = find_low_from_right(&batteries) {
                if batteries.len() == batteries_count {
                    break;
                }

                batteries.remove(same.0);
                println!("Removed from right: {:?} :: {:?}", same, batteries);
            }
        }

        if batteries.len() > batteries_count {
            while let Some(same) = find_same(&batteries) {
                if batteries.len() == batteries_count {
                    break;
                }

                batteries.remove(same.0);
                println!("Removed duplicate: {:?} :: {:?}", same, batteries);
            }
        }

        let mut jolts = 0;

        for n in batteries {
            jolts *= 10;
            jolts += n;
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

pub fn find_strongest(nums: &Vec<usize>) -> (usize, usize) {
    let mut strongest = (0, nums[0]);

    for i in 1..nums.len() {
        if nums[i] > strongest.1 || (nums[i] == strongest.1 && i > strongest.0) {
            strongest = (i, nums[i]);
        }
    }

    strongest
}

pub fn find_low_from_left(nums: &Vec<usize>) -> Option<(usize, usize)> {
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            return Some((i - 1, nums[i - 1]));
        }
    }

    None
}

pub fn find_same(nums: &Vec<usize>) -> Option<(usize, usize)> {
    for i in (0..nums.len()).rev() {
        if nums[i] == nums[i - 1] {
            return Some((i, nums[i]));
        }
    }

    None
}

pub fn find_low_from_right(nums: &Vec<usize>) -> Option<(usize, usize)> {
    for i in (1..nums.len()).rev() {
        if nums[i] < nums[i - 1] {
            return Some((i, nums[i]));
        }
    }

    None
}

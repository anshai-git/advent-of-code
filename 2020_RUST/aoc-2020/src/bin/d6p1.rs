use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

use aoc_2020::lib::util::get_file;

fn main() {
    let sum: usize = BufReader::new(&get_file().unwrap())
        .lines()
        .flatten()
        .collect::<Vec<String>>()
        .split(|l| l.is_empty())
        .map(|g| g.to_vec().join("").chars().collect::<HashSet<char>>().len())
        .sum();
    
    println!("groups: {:?}", sum);
}

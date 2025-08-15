use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use aoc_2020::lib::util::get_file;

fn main() {
    let sum: usize = BufReader::new(&get_file().unwrap())
        .lines()
        .flatten()
        .collect::<Vec<String>>()
        .split(|l| l.is_empty())
        .map(|g| {
            let people: Vec<Vec<char>> = g.to_vec().into_iter().map(|s| s.chars().collect::<Vec<char>>()).collect();
            let mut count_by_answer: HashMap<char, usize> = HashMap::new();
            for ansers in &people {
                for a in ansers {
                    count_by_answer.entry(*a).and_modify(|n| *n += 1).or_insert(1);
                }
            }
            count_by_answer.iter().filter(|p| p.1 == &people.len()).count()
        })
        .sum();
    println!("groups: {:?}", sum);
}

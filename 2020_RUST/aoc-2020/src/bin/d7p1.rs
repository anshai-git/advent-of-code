use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use aoc_2020::lib::util::get_file;

fn main() {
    let file: File = get_file().unwrap();

    let lines_raw: Vec<String> = BufReader::new(&file).lines().flatten().collect();
    let rules: Vec<String> = lines_raw
        .iter()
        .map(|l| {
            l.split("contain")
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>()
        })
        .filter_map(|v| {
            if v.get(1).map_or(false, |s| s.contains("shiny gold")) {
                v.get(0).cloned()
            } else {
                None
            }
        })
        .collect();

    println!("{:?}", rules);
    println!("{:?}", rules.len());
}

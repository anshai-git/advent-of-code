use std::{
    char,
    fs::File,
    io::{BufRead, BufReader},
};

use aoc_2020::lib::util::{open_file, parse_filename};

#[derive(Debug)]
struct Password {
    pub min: u8,
    pub max: u8,
    pub target: char,
    pub value: String,
}

impl From<&String> for Password {
    fn from(value: &String) -> Self {
        let parts: Vec<&str> = value.split_whitespace().collect();
        let range: Vec<u8> = parts.get(0).unwrap().split("-").map(str::trim).map(|n| n.parse::<u8>().unwrap()).collect();
        let target: char = parts.get(1).unwrap().chars().nth(0).unwrap();

        Self {
            min: range.get(0).unwrap().clone(),
            max: range.get(1).unwrap().clone(),
            target,
            value: parts.get(2).unwrap().to_string(),
        }
    }
}

impl Password {
    fn is_valid(&self) -> bool {
        let count: usize = self.value.chars().into_iter()
        .filter(|c| c == &self.target)
        .count();

        count >= self.min as usize && count <= self.max as usize
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|filename| open_file(&filename))
        .unwrap();

    let lines: Vec<String> = BufReader::new(&file).lines().flatten().collect();

    let passwords: Vec<Password> = lines.iter().map(|it| it.into()).collect();
    println!("passwords: {:?}", passwords);

    let valid_count: usize = passwords.iter().filter(|p| p.is_valid()).count();
    println!("Valid count: {}", valid_count);
}

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code::lib::util::{open_file, parse_filename};

struct SignalPart {
    value: HashSet<char>,
}

impl SignalPart {
    fn from(source: String) -> Self {
        Self {
            value: source.chars().collect::<HashSet<char>>(),
        }
    }

    fn score(&self) -> usize {
        self.value.len()
    }

    fn print(&self, index: usize) {
        println!(
            "Value: {:?}, Score: {:?}, Index: {:?}",
            self.value,
            self.score(),
            index
        );
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .unwrap();

    let line = BufReader::new(file).lines().flatten().nth(0).unwrap();
    println!("Line: {:?}", line);

    let mut result = 0;
    for i in 0..line.chars().count() {
        let end_index = i + 14;
        let raw = &line[i..end_index];
        let signal_part = SignalPart::from(raw.to_string());
        signal_part.print(end_index);
        if signal_part.score() == 14 {
            result = end_index;
            break;
        }
    }

    println!("Result: {:?}", result);
}

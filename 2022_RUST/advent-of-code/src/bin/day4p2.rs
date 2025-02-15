use advent_of_code::lib::util::{open_file, parse_filename};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Section {
    start: usize,
    end: usize,
}

impl Section {
    fn overlap(&self, section: &Section) -> bool {
        (self.start <= section.start && self.end >= section.end) ||
            (self.end >= section.start && self.start <= section.start)
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .unwrap();

    let section_pairs: Vec<(Section, Section)> = BufReader::new(file)
        .lines()
        .flatten()
        .map(|line| {
            let mut section_raw = line.split(",");
            (
                section_from_str(section_raw.nth(0).unwrap()),
                section_from_str(section_raw.nth(0).unwrap()),
            )
        })
        .collect();

    let sum = section_pairs
        .iter()
        .filter(|pair| pair.0.overlap(&pair.1) || pair.1.overlap(&pair.0))
        .count();

    println!("Result: {}", sum);
}

fn section_from_str(raw: &str) -> Section {
    let mut parts = raw.split("-");
    Section {
        start: parts.nth(0).unwrap().parse::<usize>().unwrap(),
        end: parts.nth(0).unwrap().parse::<usize>().unwrap(),
    }
}

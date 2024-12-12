use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

use aoc_2024::lib::util::{open_file, parse_filename};

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .unwrap();

    let mut stones: BTreeMap<u64, u64> = BufReader::new(&file)
        .lines()
        .nth(0)
        .unwrap()
        .ok()
        .unwrap()
        .split(" ")
        .map(|it| it.parse::<u64>())
        .flatten()
        .map(|e| (e, 1))
        .collect();

    for _ in 0..75 {
        let mut new_stones: BTreeMap<u64, u64> = BTreeMap::new();
        for (stone, count) in &stones {
            match *stone {
                0 => {
                    new_stones
                        .entry(1)
                        .and_modify(|e| *e += *count)
                        .or_insert(*count);
                }
                _ if let Some(stone_string) = has_even_number_of_digits(stone) => {
                    let mid = s.chars().count() / 2;
                    let (first, second) = stone_string.split_at(mid);
                    new_stones
                        .entry(first.parse::<u64>().unwrap())
                        .and_modify(|e| *e += *count)
                        .or_insert(*count);
                    new_stones
                        .entry(second.parse::<u64>().unwrap())
                        .and_modify(|e| *e += *count)
                        .or_insert(*count);
                }
                _ => {
                    new_stones.insert(2024 * (*stone), *count);
                }
            };
        }
        stones = new_stones;
    }

    let sum: u64 = stones.iter().map(|(_, v)| *v).sum();
    println!("Stones: {}", sum);
}

fn has_even_number_of_digits(n: &u64) -> Option<String> {
    let stone_string = n.to_string();
    if stone_string.chars().count() % 2 == 0 { Some(stone_string) } else { None }
}

/*
    218817038947400
    218817038947400
*/

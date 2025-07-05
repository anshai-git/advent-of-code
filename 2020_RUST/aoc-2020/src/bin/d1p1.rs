use std::{collections::HashSet, fs::File, io::{BufRead, BufReader, Result}};

use aoc_2020::lib::util::{open_file, parse_filename};

const TARGET: u32 = 2020;

fn main() -> Result<()> {
    let file: File = parse_filename()
        .and_then(|filename| open_file(&filename))
        .unwrap();

    let numbers: Vec<u32> = BufReader::new(&file)
        .lines()
        .into_iter()
        .map(|line| line.unwrap().to_string().parse::<u32>().unwrap())
        .collect();

    println!("{:?}", numbers);

    let mut items: HashSet<u32> = HashSet::new();

    for a in numbers {
        if let Some(b) = items.get(&(TARGET - a)) {
            println!("Result: {}", a * b);
            break;
        } else {
            items.insert(a);
        }
    }

    Ok(())
}

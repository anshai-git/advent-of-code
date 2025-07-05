use std::{collections::HashSet, fs::File, io::{BufRead, BufReader, Result}};

use aoc_2020::lib::util::{open_file, parse_filename};

const TARGET: i64 = 2020;

fn main() -> Result<()> {
    let file: File = parse_filename()
        .and_then(|filename| open_file(&filename))
        .unwrap();

    let numbers: Vec<i64> = BufReader::new(&file)
        .lines()
        .into_iter()
        .map(|line| line.unwrap().to_string().parse::<i64>().unwrap())
        .collect();

    println!("{:?}", numbers);

    let mut items: HashSet<i64> = HashSet::new();

    for i in 0..numbers.len() - 1 {
        for j in 0..numbers.len() - 1 {
            let a = numbers.get(i).unwrap();
            let b = numbers.get(j).unwrap();
            if  items.contains(&(TARGET - (a + b))) {
                println!("Result: {:?}", a * b * (TARGET - (a + b)));
                break;
            } else {
                items.insert(numbers.get(i).unwrap().clone());
            }
        }
    }

    Ok(())
}

use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    process::exit,
};

use aoc_2024::lib::util::{open_file, parse_filename};

fn main() {
    let file = parse_filename().and_then(|filename| open_file(&filename));
    if file.is_none() {
        exit(1);
    }

    let lines_raw: Vec<String> = BufReader::new(file.unwrap()).lines().flatten().collect();
    let split_index: usize = lines_raw.iter().position(|s| s.is_empty()).unwrap();
    let rules_raw = &lines_raw[..split_index];
    let updates_raw = &lines_raw[(split_index + 1)..];

    /* Process Rules */
    let mut rule_map: HashMap<usize, Vec<usize>> = HashMap::new();
    /* Will look like this:

        key :: value
        --------------------------
        75 :: [29, 53, 47, 61, 13]
        29 :: [13]
        47 :: [53, 13, 61, 29]
        61 :: [13, 53, 29]
        97 :: [13, 61, 47, 29, 53, 75]
        53 :: [29, 13]

        RULE >> every number from the value list must appear somewhere after the "key" value
    */

    for rule in rules_raw {
        if let (Some(left), Some(right)) = (
            rule.split('|').nth(0).unwrap().parse::<usize>().ok(),
            rule.split('|').nth(1).unwrap().parse::<usize>().ok(),
        ) {
            rule_map.entry(left).or_insert_with(Vec::new).push(right);
        }
    }

    let mut middle_value_sum: usize = 0;

    /* Process Updates */
    for update_raw in updates_raw.iter() {
        let mut update: Vec<usize> = update_raw
            .split(',')
            .map(|it| it.parse::<usize>())
            .flatten()
            .collect();

        let mut is_valid: bool = true;

        for current_position in 0..update.len() {
            while let Some(invalid_value) = update[0..current_position].iter().find(|e| {
                rule_map
                    .get(&update[current_position])
                    .unwrap_or(&mut Vec::new())
                    .clone()
                    .contains(e)
            }) {
                is_valid = false;
                let invalid_position_left = update.iter().position(|e| e == invalid_value).unwrap();
                update.swap(invalid_position_left, current_position);
            }
        }

        if !is_valid {
            let middle_value = update.get(update.len() / 2).unwrap();
            middle_value_sum += middle_value;
        }
    }

    println!("{}", middle_value_sum);
}

/*
    5723 :: too low
    5833 :: correct
*/

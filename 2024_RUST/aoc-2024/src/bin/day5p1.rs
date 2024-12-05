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

    for rule in rules_raw {
        if let (Some(left), Some(right)) = (
            rule.split('|').nth(0).unwrap().parse::<usize>().ok(),
            rule.split('|').nth(1).unwrap().parse::<usize>().ok(),
        ) {
            rule_map
                .entry(left)
                .or_insert_with(Vec::new)
                .push(right);
        }
    }
    let mut middle_value_sum: usize = 0;

    /* Process Updates */
    'updateloop: for update in updates_raw {
        let update_values: Vec<usize> = update
            .split(',')
            .map(|it| it.parse::<usize>())
            .flatten()
            .collect();

        for (index, uv) in update_values.iter().enumerate() {
            if let Some(rule_values) = rule_map.get(&uv) {
                if update_values[0..index].iter().any(|&e| rule_values.contains(&e)) { // invalid
                    continue 'updateloop;
                }
            }
        }
        let middle_value = update_values.get(update_values.len() / 2).unwrap();
        middle_value_sum += middle_value;
    }

    println!("{}", middle_value_sum);
}

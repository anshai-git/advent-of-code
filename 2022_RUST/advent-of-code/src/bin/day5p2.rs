use std::{
    collections::{vec_deque, HashMap, VecDeque},
    fs::File,
    io::{stdout, BufRead, BufReader},
};

use advent_of_code::lib::util::{open_file, parse_filename};

#[derive(Debug)]
struct Instruction {
    count: usize,
    source: usize,
    destination: usize,
}

impl Instruction {
    fn new(count: usize, source: usize, destination: usize) -> Self {
        Self {
            count,
            source,
            destination,
        }
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .expect("Failed to open file");

    let reader = BufReader::new(file);
    let parts_raw: Vec<Vec<String>> = reader
        .lines()
        .flatten()
        .collect::<Vec<String>>()
        .split(|s| s.is_empty())
        .map(|raw| raw.to_vec())
        .collect();

    let mut stacks: HashMap<usize, VecDeque<char>> = HashMap::new();
    for line in parts_raw.get(0).unwrap() {
        for (i, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                let stack_index = i / 4;
                println!("Add {} in stack {}", c, stack_index);
                stacks
                    .entry(stack_index)
                    .and_modify(|stack| stack.push_front(c))
                    .or_insert_with(|| VecDeque::from([c]));
            }
        }
    }

    for (k, v) in &stacks {
        println!("K :: {:?} | V :: {:?}", k, v);
    }

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in parts_raw.get(1).unwrap() {
        println!("{}", line);
        let mut line_parts = line.split(" ");
        let count = line_parts
            .nth(1)
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap();

        let source = line_parts
            .nth(1)
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap();

        let destination = line_parts
            .nth(1)
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap();

        let instruction: Instruction = Instruction::new(count, source - 1, destination - 1);
        println!("{:?}", instruction);

        instructions.push(instruction);
    }

    for (k, v) in &stacks {
        println!("K :: {:?} | V :: {:?}", k, v);
    }

    println!("====================");

    for instruction in instructions {
        let mut mid: VecDeque<char> = VecDeque::new();
        for _ in 0..instruction.count {
            let val = stacks
                .get_mut(&instruction.source)
                .unwrap()
                .pop_back()
                .unwrap();

            mid.push_back(val);

            for (k, v) in &stacks {
                println!("K :: {:?} | V :: {:?}", k, v);
            }

            println!("====================");
        }

        for item in mid.iter().rev() {
            stacks
                .get_mut(&instruction.destination)
                .unwrap()
                .push_back(*item);
        }
    }

    for (k, v) in &stacks {
        println!("K :: {:?} | V :: {:?}", k, v);
    }

    let mut result: Vec<char> = Vec::new();
    let mut entries: Vec<(usize, VecDeque<char>)> =
        stacks.into_iter().map(|(k, v)| (k, v)).collect();

    entries.sort_by_key(|e| e.0);
    for (_, v) in entries {
        let c: char = v.clone().pop_back().unwrap();
        result.push(c);
    }

    println!("{}", result.iter().collect::<String>());
}

use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn print_usage(program: &String) -> () {
    println!("Usage: {program} <filename>");
}

fn get_program_name() -> String {
    args().next().unwrap()
}

fn get_filename(program: &String) -> String {
    match args().count() {
        2 => args().nth(1).unwrap(),
        _ => {
            println!("Invalid number of arguments.");
            print_usage(program);
            exit(1);
        }
    }
}

fn read_file(filename: String) -> File {
    match File::open(&filename) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read file: '{filename}'");
            exit(1);
        }
    }
}

fn main() {
    let program: String = get_program_name();
    let filename: String = get_filename(&program);
    let file: File = read_file(filename);
    let reader: BufReader<File> = BufReader::new(file);
    let mut lines = reader.lines();

    let directions: Vec<Direction> = lines
        .nth(0)
        .unwrap()
        .map(String::from)
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => {
                        eprintln!("Invalid character: {}", c);
                        exit(1);
                    }
                })
                .collect()
        })
        .unwrap();

    let mut value_map: HashMap<String, (String, String)> = HashMap::new();
    for line in lines.into_iter().skip(1) {
        if let Ok(line) = line {
            if let Some((key, values)) = line.split_once('=') {
                let key: String = key.trim().to_string();
                let raw_map: (&str, &str) = values
                    .trim()
                    .trim_matches(&['(', ')'][..])
                    .split_once(',')
                    .unwrap();
                let map: (String, String) =
                    (raw_map.0.trim().to_string(), raw_map.1.trim().to_string());
                value_map.insert(key, map);
            }
        }
    }

    let mut cycle_directions = directions.iter().cycle();
    let mut next_map = "AAA";

    let mut cycle_count = 0;
    while next_map.ne("ZZZ") {
        cycle_count += 1;
        match cycle_directions.next().unwrap() {
            Direction::Left => next_map = &value_map.get(next_map).unwrap().0,
            Direction::Right => next_map = &value_map.get(next_map).unwrap().1,
        };
    }

    println!("CYCCLE COUNT: {}", cycle_count);
}

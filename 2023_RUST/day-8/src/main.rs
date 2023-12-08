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
    let mut current_maps: Vec<&String> = value_map.keys().filter(|k| k.ends_with('A')).collect();
    let mut cycle_counts: Vec<u64> = Vec::new();

    let mut cycle_count: u64 = 0;
    for ele in current_maps.iter_mut() {
        while !ele.ends_with('Z') {
            cycle_count += 1;
            let dir: &Direction = cycle_directions.next().unwrap();
            match dir {
                Direction::Left => *ele = &mut &value_map.get(*ele).unwrap().0,
                Direction::Right => *ele = &mut &value_map.get(*ele).unwrap().1,
            };
        }
        cycle_counts.push(cycle_count);
        cycle_count = 0;
    }

    let result = cycle_counts.iter().cloned().fold(1, lcm);
    println!("\nResult: {}", result);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

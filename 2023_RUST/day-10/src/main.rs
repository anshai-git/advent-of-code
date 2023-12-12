use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

#[derive(Debug)]
enum Direction {
    Top,
    Bottom,
    Right,
    Left,
}

fn print_usage(program: String) -> () {
    println!("Usage: {program} <filename>");
}

fn main() {
    let program: String = args().next().unwrap();
    let filename: String = match args().count() {
        2 => args().nth(1).unwrap(),
        _ => {
            eprintln!("Invalid number of arguments.");
            print_usage(program);
            exit(1);
        }
    };

    let file: File = match File::open(&filename) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to open file {filename}");
            exit(1);
        }
    };

    let reader: BufReader<File> = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in reader.lines().flatten() {
        map.push(line.chars().collect());
    }

    let mut starting_position: (u32, u32) = (0, 0);
    for (index, line) in map.iter().enumerate() {
        if let Some(char_index) = line.iter().position(|&c| c == 'S') {
            starting_position = (index as u32, char_index as u32);
            break;
        }
    }

    let mut current_position = starting_position;
    let mut current_step;

    if ['|', '7', 'F'].contains(&map[current_position.0 as usize - 1][current_position.1 as usize])
    {
        current_step = Direction::Top;
    } else if ['|', 'J', 'L']
        .contains(&map[current_position.0 as usize + 1][current_position.1 as usize])
    {
        current_step = Direction::Bottom;
    } else if ['-', 'F', 'L']
        .contains(&map[current_position.0 as usize][current_position.1 as usize - 1])
    {
        current_step = Direction::Left;
    } else {
        current_step = Direction::Right;
    }

    loop {
        match current_step {
            Direction::Top => {
                current_position.0 -= 1;
            }
            Direction::Bottom => {
                current_position.0 += 1;
            }
            Direction::Left => {
                current_position.1 -= 1;
            }
            Direction::Right => {
                current_position.1 += 1;
            }
        }

        match map[current_position.0 as usize][current_position.1 as usize] {
            '-' => current_step = Direction::Right,
            _ => {
                eprintln!("Invalid character.");
            }
        }

        if current_position == starting_position {
            break;
        }
    }

    println!("{:?}", starting_position);
    println!("{:?}", current_step);
}

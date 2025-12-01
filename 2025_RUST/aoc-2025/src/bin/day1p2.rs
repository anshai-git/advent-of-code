use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use aoc_2025::lib::utils::{open_file, parse_filename};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation(Direction, isize);

impl From<String> for Rotation {
    fn from(value: String) -> Self {
        let direction: Direction = match value.chars().nth(0).unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => {
                panic!("Invalid rotation string.");
            }
        };
        let distance: isize = value[1..].parse::<isize>().unwrap();

        Rotation(direction, distance)
    }
}

pub fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .expect("Error opening file");

    let rotations: Vec<Rotation> = BufReader::new(file)
        .lines()
        .into_iter()
        .map(|l| l.unwrap().to_string())
        .map(|l| Rotation::from(l))
        .collect();

    let mut position: isize = 50;
    let mut zero_count = 0;

    for rotation in rotations {
        let distance = if rotation.1 > 100 {
            zero_count += rotation.1 / 100;
            rotation.1 % 100
        } else {
            rotation.1
        };

        match rotation.0 {
            Direction::Left => {
                if (position - distance) < 0 {
                    if position != 0 && position - distance < 0 && position - distance != 0 {
                        zero_count += 1;
                    }
                    position = 100 + (position - distance);
                } else {
                    position = position - distance;
                }
            }
            Direction::Right => {
                if (position + distance) > 99 {
                    if position + distance > 0 && position + distance != 100 {
                        zero_count += 1;
                    }
                    position = (position + distance) - 100;
                } else {
                    position = position + distance;
                }
            }
        }

        println!("{:?} :: {}", rotation, position);

        if position == 0 {
            zero_count += 1;
        }
    }

    println!("Zero count: {}", zero_count);
}

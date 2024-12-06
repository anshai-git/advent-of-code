use std::{
    collections::HashSet, io::{BufRead, BufReader}, process::exit
};

use aoc_2024::lib::util::{open_file, parse_filename};

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    row: usize,
    col: usize,
    direction: Direction,
}

impl Position {
    fn new(row: usize, col: usize, direction: Direction) -> Self {
        Self {
            row,
            col,
            direction,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn main() {
    let file = parse_filename().and_then(|filename| open_file(&filename));
    if file.is_none() {
        exit(1);
    }

    let mut map: Vec<Vec<char>> = BufReader::new(file.unwrap())
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect();

    let mut current_position: Position = find_starting_position(&map);
    println!("Startin Position: {:?}", current_position);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((current_position.row, current_position.col));

    let mut steps_count: usize = 0;
    while let Some(next_position) = step(&mut map, &current_position) {
        println!("Next Position: {:?}", next_position);
        current_position = next_position;

        if !visited.contains(&(current_position.row, current_position.col)) {
            steps_count += 1;
            visited.insert((current_position.row, current_position.col));
        }

        println!("Steps: {}", steps_count);
    }

    println!("Steps Count: {}", steps_count);
}

fn step(map: &mut [Vec<char>], current_position: &Position) -> Option<Position> {
    print_map(map);

    let mut next_position: (usize, usize) = match current_position.direction {
        Direction::Up => (current_position.row - 1, current_position.col),
        Direction::Down => (current_position.row + 1, current_position.col),
        Direction::Left => (current_position.row, current_position.col - 1),
        Direction::Right => (current_position.row, current_position.col + 1),
    };

    if next_position.0 >= map.len()
        || next_position.0 < 0 // useless due to type limits
        || next_position.1 >= map[0].len()
        || next_position.1 < 0 // useless due to type limits
    {
        return None
    }

    let new_direction: Option<Direction> = if map[next_position.0][next_position.1] == '#' {
        Some(current_position.direction.next())
    } else {
        None
    };

    let result = if let Some(nd) = new_direction {
        next_position = match nd {
            Direction::Up => (current_position.row - 1, current_position.col),
            Direction::Down => (current_position.row + 1, current_position.col),
            Direction::Left => (current_position.row, current_position.col - 1),
            Direction::Right => (current_position.row, current_position.col + 1),
        };
        Some(Position::new(next_position.0, next_position.1, nd))
    } else {
        Some(Position::new(next_position.0, next_position.1, current_position.direction.clone()))
    };

    println!("");
    if let Some(pos) = &result {
        map[current_position.row][current_position.col] = '.';
        map[pos.row][pos.col] = 'x';
    }

    result
}

fn find_starting_position(map: &[Vec<char>]) -> Position {
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            let direction: Option<Direction> = match map[row][col] {
                '>' => Some(Direction::Left),
                '<' => Some(Direction::Right),
                '^' => Some(Direction::Up),
                'V' | 'v' => Some(Direction::Down),
                _ => None,
            };
            if let Some(dir) = direction {
                return Position::new(row, col, dir);
            }
        }
    }

    eprint!("Starting position not found");
    exit(1);
}

fn print_map(map: &[Vec<char>]) {
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            print!("{}", map[row][col])
        }
        println!("");
    }
}

/*
    5403 :: need to add one for the last position >> 5404
*/

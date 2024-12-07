use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
    process::exit,
};

use aoc_2024::lib::util::{open_file, parse_filename};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    fn next(&self) -> Self {
        match self.direction {
            Direction::Up => Self {
                row: self.row - 1,
                col: self.col,
                direction: self.direction,
            },
            Direction::Down => Self {
                row: self.row + 1,
                col: self.col,
                direction: self.direction,
            },
            Direction::Left => Self {
                row: self.row,
                col: self.col - 1,
                direction: self.direction,
            },
            Direction::Right => Self {
                row: self.row,
                col: self.col + 1,
                direction: self.direction,
            },
        }
    }

    fn next_dir(&self) -> Self {
        match self.direction.next() {
            Direction::Up => Self {
                row: self.row - 1,
                col: self.col,
                direction: self.direction,
            },
            Direction::Down => Self {
                row: self.row + 1,
                col: self.col,
                direction: self.direction,
            },
            Direction::Left => Self {
                row: self.row,
                col: self.col - 1,
                direction: self.direction,
            },
            Direction::Right => Self {
                row: self.row,
                col: self.col + 1,
                direction: self.direction,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    let starting_position: Position = find_starting_position(&map);
    let mut current_position: Position = starting_position.clone();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((current_position.row, current_position.col));

    while let Some(next_position) = step(&mut map, &current_position) {
        current_position = next_position;
        if !visited.contains(&(current_position.row, current_position.col)) {
            visited.insert((current_position.row, current_position.col));
        }
    }
    map[current_position.row][current_position.col] = '.';

    let mut infinite_loop_count = 0;
    let mut infinite_loop_obstacles: Vec<(usize, usize)> = Vec::new();
    for item in visited {
        let mut current = starting_position.clone();
        let mut visited_positions: HashSet<Position> = HashSet::new();
        map[item.0][item.1] = '#';
        'map_check: while let Some(next_position) = step(&mut map, &current) {
            if !visited_positions.insert(current.clone()) {
                infinite_loop_obstacles.push(item);
                infinite_loop_count += 1;
                break 'map_check;
            }

            current = next_position;
        }

        map[current.row][current.col] = '.';
        map[item.0][item.1] = '.';
    }

    println!("Valid Positions: {}", infinite_loop_count);
}

fn step(map: &mut [Vec<char>], current_position: &Position) -> Option<Position> {
    println!("");
    print_map(&map);
    println!("");
    if (current_position.direction == Direction::Up && current_position.row == 0)
        || (current_position.direction == Direction::Left && current_position.col == 0)
    {
        return None;
    }

    let mut next = current_position.next();
    while map[next.row][next.col] == '#' {
        next = next.next_dir();
    }

    // let mut next_position_opt: Option<(usize, usize)> = match current_position.direction {
    //     Direction::Up => { if current_position.row == 0 { None } else { Some((current_position.row - 1, current_position.col)) } }
    //     Direction::Down => Some((current_position.row + 1, current_position.col)),
    //     Direction::Left => { if current_position.col == 0 { None } else { Some((current_position.row, current_position.col - 1)) } }
    //     Direction::Right => Some((current_position.row, current_position.col + 1)),
    // };

    // if next_position_opt.is_none() {
    //     return None;
    // }

    // let mut next_position = next_position_opt.unwrap();

    // if next_position.0 >= map.len() || next_position.1 >= map[0].len() {
    //     return None;
    // }

    // let mut result: Position = Position::new(
    //     next_position.0,
    //     next_position.1,
    //     current_position.direction.clone(),
    // );

    // let mut flag = 0;
    // let mut current_direction: Direction = current_position.direction.clone();
    // while map[next_position.0][next_position.1] == '#' {
    //     if flag >= 10 {
    //         break;
    //     }
    //     current_direction = current_direction.next();
    //     next_position_opt = match current_direction {
    //         Direction::Up => {
    //             if current_position.row == 0 {
    //                 None
    //             } else {
    //                 Some((current_position.row - 1, current_position.col))
    //             }
    //         }
    //         Direction::Down => Some((current_position.row + 1, current_position.col)),
    //         Direction::Left => {
    //             if current_position.col == 0 {
    //                 None
    //             } else {
    //                 Some((current_position.row, current_position.col - 1))
    //             }
    //         }
    //         Direction::Right => Some((current_position.row, current_position.col + 1)),
    //     };

    //     if next_position_opt.is_some() {
    //         next_position = next_position_opt.unwrap();
    //     }

    //     result = Position::new(next_position.0, next_position.1, current_direction);
    //     flag += 1;
    // }

    map[current_position.row][current_position.col] = '.';
    map[next.row][next.col] = 'x';

    Some(next)
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
    5092
*/

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use aoc_2024::lib::util::{open_file, parse_filename};

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn apply(&self, movement: &Movement) -> Position {
        match movement {
            Movement::Up => Position::new(self.x, self.y - 1),
            Movement::Down => Position::new(self.x, self.y + 1),
            Movement::Left => Position::new(self.x - 1, self.y),
            Movement::Right => Position::new(self.x + 1, self.y),
        }
    }
}

#[derive(Debug)]
enum Movement {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
enum Cell {
    Wall,
    Box,
    Empty,
}

impl Cell {
    fn pretty(&self) -> String {
        match self {
            Cell::Box => String::from("O"),
            Cell::Empty => String::from("."),
            Cell::Wall => String::from("#"),
        }
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .unwrap();

    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();

    let mut parts = lines.split(|l| l.is_empty());
    let mut current_pos: Position = Position::new(0, 0);

    let mut map: Vec<Vec<Cell>> = parts
        .nth(0)
        .unwrap()
        .iter()
        .enumerate()
        .map(|(line_index, line_raw)| {
            line_raw
                .chars()
                .enumerate()
                .map(|(c_index, c)| match c {
                    '#' => Cell::Wall,
                    'O' => Cell::Box,
                    '@' => {
                        current_pos = Position::new(c_index, line_index);
                        Cell::Empty
                    }
                    _ => Cell::Empty,
                })
                .collect()
        })
        .collect();

    let movements: Vec<Movement> = parts
        .nth(0)
        .unwrap()
        .iter()
        .map(|l| {
            l.chars().map(|c| match c {
                '>' => Movement::Right,
                '<' => Movement::Left,
                '^' => Movement::Up,
                _ => Movement::Down,
            })
        })
        .flatten()
        .collect();

    // print_map(&map, &current_pos);
    // println!("ROBOT: {:?}", &current_pos);
    // println!("{:?}", &movements);

    for movement in &movements {
        // println!("Movement: {:?}", &movement);
        let next_pos = current_pos.apply(&movement);

        match map[next_pos.y][next_pos.x] {
            Cell::Box => {
                if let Some(empty_position) = find_empty(&map, &next_pos, &movement) {
                    shift_boxes(&mut map, &next_pos, &empty_position);
                    current_pos = next_pos;
                }
            }
            Cell::Empty => {
                current_pos = next_pos;
            }
            Cell::Wall => {}
        }

        // print_map(&map, &current_pos);
    }

    let mut sum: u64 = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if let Cell::Box = map[y][x] {
                sum += 100 * y as u64 + x as u64;
            }
        }
    }

    println!("SUM: {}", sum);
}

fn find_empty(map: &Vec<Vec<Cell>>, position: &Position, movement: &Movement) -> Option<Position> {
    let mut curr = position.clone();
    loop {
        curr = curr.apply(movement);
        match map[curr.y][curr.x] {
            Cell::Empty => return Some(curr),
            Cell::Wall => return None,
            _ => continue,
        }
    }
}

/* We don't even need to shift all the boxes, it's enaugh if we move the first box to the end of the boxes in the empty space */
fn shift_boxes(map: &mut Vec<Vec<Cell>>, start: &Position, end: &Position) {
    map[start.y][start.x] = Cell::Empty;
    map[end.y][end.x] = Cell::Box;
}

fn print_map(map: &Vec<Vec<Cell>>, robot: &Position) {
    for (row_index, row) in map.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if row_index == robot.y && col_index == robot.x {
                print!("@");
            } else {
                print!("{}", col.pretty());
            }
        }
        println!("");
    }
}

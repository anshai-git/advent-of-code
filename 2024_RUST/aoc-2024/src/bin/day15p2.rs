use std::{
    collections::HashSet, fs::File, io::{BufRead, BufReader}
};

use aoc_2024::lib::util::{open_file, parse_filename};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

#[derive(Debug, PartialEq)]
enum Movement {
    Left,
    Right,
    Up,
    Down,
}

impl Movement {
    fn opposite(&self) -> Self {
        match self {
            Movement::Down => Movement::Up,
            Movement::Up => Movement::Down,
            Movement::Left => Movement::Right,
            Movement::Right => Movement::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    BoxR,
    BoxL,
    Empty,
}

impl Cell {
    fn pretty(&self) -> String {
        match self {
            Cell::BoxL => String::from("["),
            Cell::BoxR => String::from("]"),
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
                    '#' => vec![Cell::Wall, Cell::Wall],
                    'O' => vec![Cell::BoxL, Cell::BoxR],
                    '@' => {
                        current_pos = Position::new(c_index * 2, line_index);
                        vec![Cell::Empty, Cell::Empty]
                    }
                    _ => vec![Cell::Empty, Cell::Empty],
                })
                .flatten()
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
    for movement in &movements {
        // println!("Movement: {:?}", &movement);
        let next_pos = current_pos.apply(&movement);

        match map[next_pos.y][next_pos.x] {
            Cell::BoxL | Cell::BoxR => match movement {
                Movement::Left | Movement::Right => {
                    if let Some(empty_position) = find_empty_horizontally(&map, &next_pos, &movement)
                    {
                        shift_boxes_horizontally(&mut map, &next_pos, &empty_position, &movement);
                        current_pos = next_pos;
                    }
                }
                Movement::Up | Movement::Down => {
                    let mut current = vec![next_pos.clone(), find_pair(&map, &next_pos)];
                    let mut layers = vec![current.clone()];

                    loop {
                        let collisiones = find_collisions(current.clone(), &movement);
                        if collisiones.iter().any(|p| map[p.y][p.x] == Cell::Wall) {
                            break;
                        }

                        if collisiones.iter().any(|p| map[p.y][p.x] == Cell::BoxL || map[p.y][p.x] == Cell::BoxR) {
                            let all_with_pairs: HashSet<Position> = collisiones
                                .iter()
                                .filter(|p| map[p.y][p.x] == Cell::BoxL || map[p.y][p.x] == Cell::BoxR)
                                .map(|p| vec![p.clone(), find_pair(&map, p)])
                                .flatten()
                                .collect::<HashSet<Position>>();

                            let deduplicated: Vec<Position> = all_with_pairs.clone().into_iter().collect();
                            layers.push(deduplicated.clone());
                            current = deduplicated;
                            continue;
                        }

                        if collisiones.iter().all(|p| map[p.y][p.x] == Cell::Empty) {
                            for layer in layers.iter().rev() {
                                for position in layer {
                                    let future_pos = position.apply(movement);
                                    map[future_pos.y][future_pos.x] = map[position.y][position.x];
                                    map[position.y][position.x] = Cell::Empty;
                                }
                            }

                            current_pos = next_pos;
                            break;
                        }
                    }
                }
            },
            Cell::Empty => {
                current_pos = next_pos;
            }
            Cell::Wall => {}
        }

        // print_map(&map, &current_pos);
        // pause();
    }

    let mut sum: u64 = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if let Cell::BoxL = map[y][x] {
                sum += 100 * y as u64 + x as u64;
            }
        }
    }

    println!("SUM: {}", sum);
}

fn find_collisions(positions: Vec<Position>, movement: &Movement) -> Vec<Position> {
    positions.iter().map(|p| p.apply(movement)).collect()
}

fn find_empty_horizontally(map: &Vec<Vec<Cell>>, position: &Position, movement: &Movement) -> Option<Position> {
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

fn find_pair(map: &Vec<Vec<Cell>>, pos: &Position) -> Position {
    match map[pos.y][pos.x] {
        Cell::BoxL => Position::new(pos.x + 1, pos.y),
        Cell::BoxR => Position::new(pos.x - 1, pos.y),
        _ => unreachable!("Unreachable theoretically {:?}", pos),
    }
}

fn shift_boxes_horizontally(map: &mut Vec<Vec<Cell>>, start: &Position, end: &Position, movement: &Movement) {
    let mut current = end.clone();
    while current.x != start.x {
        map[current.y][current.x] = if *movement == Movement::Left {
            map[current.y][current.x + 1]
        } else {
            map[current.y][current.x - 1]
        };
        current = current.apply(&movement.opposite());
    }
    map[current.y][current.x] = Cell::Empty;
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

fn pause() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use aoc_2024::lib::util::{open_file, parse_filename};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
    d: Direction,
}

impl Position {
    fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    fn go(&self, d: Direction) -> Self {
        match d {
            Direction::Down => Position::new(self.x, self.y + 1, d),
            Direction::Up => Position::new(self.x, self.y - 1, d),
            Direction::Left => Position::new(self.x - 1, self.y, d),
            Direction::Right => Position::new(self.x + 1, self.y, d),
        }
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .unwrap();

    let map: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect();


    let mut pairs: Vec<(HashSet<(usize, usize, char, Direction)>, HashSet<(usize, usize)>)> = Vec::new();
    for (row_index, row) in map.clone().iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if pairs.iter().any(|(_, area)| area.iter().any(|a| *a == (row_index, col_index))) {
                println!("Already scanned: {} :: {}", row_index, col_index);
            } else {
                let c = map[row_index][col_index];
                println!("Not Scanned YET: {} :: {} :: {}", row_index, col_index, c);
                let (fence_map, area) = process_area(&map, row_index, col_index, c);
                pairs.push((fence_map, area));
            }
        }
    }


    let mut sum = 0;
    for (fence_map, area) in pairs {
        let val = fence_map.len() * area.len();
        println!("{} :: {} * {} = {}", fence_map.iter().nth(0).unwrap().2, fence_map.len(), area.len(), val);
        sum += val;
        println!("Fence Map:");
        for item in fence_map {
            println!("{:?}", item);
        }

        println!("Area Map:");
        for a in area.iter() {
            println!("{:?}", a);
        }
    }
    println!("\nSUM: {}\n", sum);
}

fn process_area(map: &Vec<Vec<char>>, row_index: usize, col_index: usize, c: char) -> (HashSet<(usize, usize, char, Direction)>, HashSet<(usize, usize)>) {
    let start_pos: Position = Position::new(col_index as i32, row_index as i32, Direction::Right);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut fence_map: HashSet<(usize, usize, char, Direction)> = HashSet::new();
    let mut area: HashSet<(usize, usize)> = HashSet::new();

    fn step(
        map: &Vec<Vec<char>>,
        position: Position,
        previous_pos: Option<Position>,
        visited: &mut HashSet<(usize, usize)>,
        fence_map: &mut HashSet<(usize, usize, char, Direction)>,
        area: &mut HashSet<(usize, usize)>,
        c: char
    ) {
        print_map_with_pos(map, position);
        if position.x < 0
            || position.y < 0
            || position.y as usize >= map.len()
            || position.x as usize >= map[0].len()
        {
            if let Some(prev) = previous_pos {
                let prev_char: char = map[prev.y as usize][prev.x as usize];
                let plot_field = (prev.y as usize, prev.x as usize, prev_char, position.d);
                println!("bounds limit {:?}", plot_field);
                fence_map.insert(plot_field);
                // pause();
            }
            return;
        }

        if map[position.y as usize][position.x as usize] == c {
            area.insert((position.y as usize, position.x as usize));
        }

        if let Some(prev) = previous_pos {
            let current_char: char = map[position.y as usize][position.x as usize];
            let previous_char: char = map[prev.y as usize][prev.x as usize];

            if current_char != previous_char {
                let plot_field = (prev.y as usize, prev.x as usize, previous_char, position.d);
                println!("char diff {:?}", plot_field);
                fence_map.insert(plot_field);
                // pause();
                return;
            }
        }

        if visited.contains(&(position.y as usize, position.x as usize)) {
            return;
        }
        visited.insert((position.y as usize, position.x as usize));
        step(map, position.go(Direction::Down), Some(position), visited, fence_map, area, c);
        step(map, position.go(Direction::Right), Some(position), visited, fence_map, area, c);
        step(map, position.go(Direction::Up), Some(position), visited, fence_map, area, c);
        step(map, position.go(Direction::Left), Some(position), visited, fence_map, area, c);
    }

    step(map, start_pos, None, &mut visited, &mut fence_map, &mut area, c);

    (fence_map, area)
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        println!("{:?}", row);
    }
}

fn print_map_with_pos(map: &Vec<Vec<char>>, position: Position) {
    println!("===============================");
    println!("{:?}", position);
    println!("===============================");
    for (row_index, row) in map.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            let c: char = if row_index == position.y as usize && col_index == position.x as usize {
                match position.d {
                    Direction::Up => '^',
                    Direction::Down => 'v',
                    Direction::Left => '<',
                    Direction::Right => '>',
                }
            } else {
                *col
            };
            print!("{c}");
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

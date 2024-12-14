use std::{
    collections::{HashMap, HashSet},
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

    let mut pairs: Vec<(
        Vec<(usize, usize, char, Direction)>,
        HashSet<(usize, usize)>,
    )> = Vec::new();
    for (row_index, row) in map.clone().iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if pairs
                .iter()
                .any(|(_, area)| area.iter().any(|a| *a == (row_index, col_index)))
            {
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
        let mut sides = 0;

        let down_sides = shrink_h_fences(&fence_map, Direction::Down);
        println!("Down Sides: {}", down_sides);
        let up_sides = shrink_h_fences(&fence_map, Direction::Up);
        println!("Up Sides: {}", up_sides);
        let left_sides = shrink_v_fences(&fence_map, Direction::Left);
        println!("Left Sides: {}", left_sides);
        let right_sides = shrink_v_fences(&fence_map, Direction::Right);
        println!("right Sides: {}", right_sides);

        sides += down_sides;
        sides += up_sides;
        sides += left_sides;
        sides += right_sides;
        let val = sides * area.len();

        println!(
            "{} :: {} * {} = {}",
            fence_map.iter().nth(0).unwrap().2,
            sides,
            area.len(),
            val
        );

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

fn shrink_h_fences(fences: &Vec<(usize, usize, char, Direction)>, direction: Direction) -> usize {
    let mut grouped: HashMap<usize, Vec<Vec<(usize, usize, char, Direction)>>> = HashMap::new();

    let mut groups_by_row: HashMap<usize, Vec<(usize, usize, char, Direction)>> = HashMap::new();
    for item in fences.iter().filter(|it| it.3 == direction) {
        groups_by_row.entry(item.0).or_default().push(*item);
    }

    for (key, mut group) in groups_by_row {
        group.sort_by(|a, b| a.1.cmp(&b.1));

        let mut subgroups = Vec::new();
        let mut current_subgroup = vec![group[0].clone()];

        for i in 1..group.len() {
            if group[i].1 - group[i - 1].1 == 1 {
                current_subgroup.push(group[i].clone());
            } else {
                subgroups.push(current_subgroup);
                current_subgroup = vec![group[i].clone()];
            }
        }
        subgroups.push(current_subgroup);
        grouped.insert(key, subgroups);
    }

    println!("{:?}", grouped);

    grouped.iter().map(|it| it.1.len()).sum()
}

fn shrink_v_fences(fences: &Vec<(usize, usize, char, Direction)>, direction: Direction) -> usize {
    let mut grouped: HashMap<usize, Vec<Vec<(usize, usize, char, Direction)>>> = HashMap::new();

    let mut groups_by_col: HashMap<usize, Vec<(usize, usize, char, Direction)>> = HashMap::new();
    for item in fences.iter().filter(|it| it.3 == direction) {
        groups_by_col.entry(item.1).or_default().push(*item);
    }

    for (key, mut group) in groups_by_col {
        group.sort_by(|a, b| a.0.cmp(&b.0));

        let mut subgroups = Vec::new();
        let mut current_subgroup = vec![group[0].clone()];

        for i in 1..group.len() {
            if group[i].0 - group[i - 1].0 == 1 {
                current_subgroup.push(group[i].clone());
            } else {
                subgroups.push(current_subgroup);
                current_subgroup = vec![group[i].clone()];
            }
        }
        subgroups.push(current_subgroup);
        grouped.insert(key, subgroups);
    }

    grouped.iter().map(|it| it.1.len()).sum()
}

fn process_area(
    map: &Vec<Vec<char>>,
    row_index: usize,
    col_index: usize,
    c: char,
) -> (
    Vec<(usize, usize, char, Direction)>,
    HashSet<(usize, usize)>,
) {
    let start_pos: Position = Position::new(col_index as i32, row_index as i32, Direction::Right);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut fence_map: Vec<(usize, usize, char, Direction)> = Vec::new();
    let mut area: HashSet<(usize, usize)> = HashSet::new();

    fn step(
        map: &Vec<Vec<char>>,
        position: Position,
        previous_pos: Option<Position>,
        visited: &mut HashSet<(usize, usize)>,
        fence_map: &mut Vec<(usize, usize, char, Direction)>,
        area: &mut HashSet<(usize, usize)>,
        c: char,
    ) {
        // print_map_with_pos(map, position);
        if position.x < 0
            || position.y < 0
            || position.y as usize >= map.len()
            || position.x as usize >= map[0].len()
        {
            if let Some(prev) = previous_pos {
                let prev_char: char = map[prev.y as usize][prev.x as usize];
                let plot_field = (prev.y as usize, prev.x as usize, prev_char, position.d);
                println!("bounds limit {:?}", plot_field);
                fence_map.push(plot_field);
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
                fence_map.push(plot_field);
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
        step(map, position.go(Direction::Left), Some(position), visited, fence_map, area, c);
        step(map, position.go(Direction::Up), Some(position), visited, fence_map, area, c);
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

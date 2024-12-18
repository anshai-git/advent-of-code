use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use aoc_2024::lib::util::{open_file, parse_filename};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Position {
    fn new(x: usize, y: usize, dir: Direction) -> Self {
        Self { x, y, dir }
    }

    fn apply(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self::new(self.x, self.y - 1, dir),
            Direction::Down => Self::new(self.x, self.y + 1, dir),
            Direction::Left => Self::new(self.x - 1, self.y, dir),
            Direction::Right => Self::new(self.x + 1, self.y, dir),
        }
    }

    fn pretty(&self) -> char {
        match self.dir {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }

    fn previous(&self) -> Self {
        let opposite_direction = self.dir.opposite();
        self.apply(opposite_direction)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .unwrap();

    let reader = BufReader::new(file);
    let mut map: Vec<Vec<char>> = reader
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect();

    let mut start_pos = Position::new(0, 0, Direction::Right);
    let mut target_pos = Position::new(0, 0, Direction::Up);

    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 'S' {
                start_pos = Position::new(x, y, Direction::Right);
            }

            if *col == 'E' {
                target_pos = Position::new(x, y, Direction::Right);
            }
        }
    }

    // let paths = find_paths(&mut map, start_pos);
    let paths = find_paths(&mut map, start_pos);

    // for (path_index, path) in paths.iter().enumerate() {
    //     println!("\n\nPATH {} :: SCORE {}", path_index, calculate_score(path));
    //     print_path(&map, path);
    // }

    let best_score: usize = paths.iter().map(|p| calculate_score(p)).min().unwrap();
    println!("\n\nBEST SCORE: {}", best_score);

    // for path in &paths {
    //     println!("{:?}", path);
    // }

    let tiles = paths
        .iter()
        .filter(|p| calculate_score(p) == best_score)
        .flatten()
        .map(|p| (p.x, p.y))
        .collect::<HashSet<(usize, usize)>>()
        .len();

    println!(
        "PATHS: {}",
        paths
            .iter()
            .filter(|p| calculate_score(p) == best_score)
            .count()
    );
    println!("TILES: {}", tiles);
}

fn find_paths(map: &mut Vec<Vec<char>>, start_pos: Position) -> Vec<Vec<Position>> {
    let mut scores: HashMap<(usize, usize), usize> = HashMap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut paths: Vec<Vec<Position>> = Vec::new();
    let mut path: Vec<Position> = Vec::new();
    let min_paths: Vec<&Vec<Position>> = Vec::new();
    let mut bp: Vec<(usize, usize)> = Vec::new();

    fn step(
        map: &mut Vec<Vec<char>>,
        position: Position,
        visited: &mut HashSet<(usize, usize)>,
        paths: &mut Vec<Vec<Position>>,
        path: &mut Vec<Position>,
        scores: &mut HashMap<(usize, usize), usize>,
        start_pos: &Position,
        best_paths: &mut Vec<(usize, usize)>,
    ) {
        if !is_in_bounds(&position, map) {
            return;
        }

        let current_score = calculate_score(path);
        let best_score_opt = paths.iter().min_by_key(|p| calculate_score(p)).and_then(|p| Some(calculate_score(p)));

        if let Some(sc) = scores.get(&(position.x, position.y)) {
            if current_score > *sc {
                // if !best_paths.contains(&(position.x, position.y)) {
                    return;
                // }
            } else {
                scores.remove(&(position.x, position.y));
                scores.insert((position.x, position.y), current_score);
            }
        } else {
            scores.insert((position.x, position.y), current_score);
        }

        // println!("CURRENT SCORE: {:?}", current_score);
        // print_path(map, path);
        // pause();

        if let Some(bs) = best_score_opt {
            if current_score > bs {
                return;
            }
        }

        if map[position.y][position.x] == 'E' {
            paths.push(path.clone());

            // if let Some(bs) = best_score_opt {
            //     let new_score = calculate_score(path);
            //     if new_score <= bs {
            //         let min_paths: Vec<_> = paths
            //             .iter()
            //             .filter(|p| calculate_score(p) == new_score)
            //             .collect();
            //         *best_paths = min_paths
            //             .iter()
            //             .flat_map(|p| p.iter())
            //             .map(|p| (p.x, p.y))
            //             .collect();
            //     }
            // } else {
            //     *best_paths = path
            //         .iter()
            //         .map(|p| (p.x, p.y))
            //         .collect();
            // }

            println!("Path found: {}", paths.len());

            return;
        }

        if visited.contains(&(position.x, position.y)) {
            return;
        }

        visited.insert((position.x, position.y));
        if map[position.y][position.x] == '#' {
            return;
        }

        path.push(position.clone());

        step(map, position.apply(Direction::Up), visited, paths, path, scores, start_pos, best_paths);
        step(map, position.apply(Direction::Right), visited, paths, path, scores, start_pos, best_paths);
        step(map, position.apply(Direction::Down), visited, paths, path, scores, start_pos, best_paths);
        step(map, position.apply(Direction::Left), visited, paths, path, scores, start_pos, best_paths);

        visited.remove(&(position.x, position.y));
        path.pop();
    }

    step(map, start_pos.clone(), &mut visited, &mut paths, &mut path, &mut scores, &start_pos, &mut bp);

    paths
}

fn is_in_bounds(position: &Position, map: &Vec<Vec<char>>) -> bool {
    position.y < map.len() && position.x < map[0].len()
}

fn print_path(map: &Vec<Vec<char>>, positions: &Vec<Position>) {
    // println!("\n\n{:?}", position);
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let c = if let Some(p) = positions.iter().find(|p| p.x == x && p.y == y) {
                p.pretty()
            } else {
                *col
            };
            print!("{}", c);
        }
        println!("");
    }
}

fn print_map(map: &Vec<Vec<char>>, position: Position) {
    // println!("\n\n{:?}", position);
    for (y, row) in map.iter().enumerate() {
        if y < map.len() / 2 {
            for (x, col) in row.iter().enumerate() {
                let c = if position.x == x && position.y == y {
                    'x'
                } else {
                    *col
                };
                print!("{}", c);
            }
            println!("");
        }
    }
}

fn pause() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}

fn calculate_score(path: &Vec<Position>) -> usize {
    let mut score: usize = 0;
    for win in path.windows(2) {
        if let [e1, e2] = win {
            if e1.dir != e2.dir {
                score += 1000;
            }
        }
    }
    score += path.len();
    score
}

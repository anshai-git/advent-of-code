use std::{
    collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader, Lines}
};

use aoc_2024::lib::util::{open_file, parse_filename};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    fn new(row: isize, col: isize) -> Self {
        Position { row, col }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Antinode {
    position: Position,
    antenna_type: char,
    antenna_a: Position,
    antenna_b: Position,
}

impl Antinode {
    fn new(
        position: Position,
        antenna_type: char,
        antenna_a: Position,
        antenna_b: Position,
    ) -> Self {
        Self {
            position,
            antenna_type,
            antenna_a,
            antenna_b,
        }
    }
}

fn main() {
    let file: File = parse_filename().and_then(|name| open_file(&name)).unwrap();
    let lines: Lines<BufReader<File>> = BufReader::new(file).lines();

    let mut height: isize = 0;
    let mut width: isize = 0;

    let mut antenna_positions: HashMap<char, Vec<Position>> = HashMap::new();
    for (row_index, line) in lines.flatten().enumerate() {
        'col_loop: for (col_index, c) in line.chars().enumerate() {
            width = col_index as isize;
            if !c.is_alphanumeric() {
                continue 'col_loop;
            }
            antenna_positions
                .entry(c)
                .or_insert_with(Vec::new)
                .push(Position {
                    row: row_index as isize,
                    col: col_index as isize,
                });
        }
        height = row_index as isize;
    }

    println!("Height: {} :: Width {}", height, width);

    let mut antinodes: HashSet<Antinode> = HashSet::new();
    for (antenna, positions) in &antenna_positions {
        println!("\n\n{} :: {:?}\n\n", antenna, positions);
        for i in 0..positions.len() {
            'j: for j in 0..positions.len() {
                if i == j {
                    continue 'j;
                }
                let pair = (positions.get(i).unwrap(), positions.get(j).unwrap());
                let antinode_row = if pair.0.row > pair.1.row {
                    pair.1.row - (pair.0.row - pair.1.row)
                } else {
                    pair.1.row + (pair.1.row - pair.0.row)
                };

                let antinode_col = if pair.0.col > pair.1.col {
                    pair.1.col - (pair.0.col - pair.1.col)
                } else {
                    pair.1.col + (pair.1.col - pair.0.col)
                };

                if antinode_row < 0
                    || antinode_col < 0
                    || antinode_row > height
                    || antinode_col > width
                {
                    continue 'j;
                }
                let antinode: Antinode = Antinode::new(
                    Position::new(antinode_row, antinode_col),
                    *antenna,
                    positions.get(i).unwrap().clone(),
                    positions.get(j).unwrap().clone(),
                );
                antinodes.insert(antinode);
                // print!("{:?} | ", pair);
            }
        }
    }

    let mut seen_positions: HashSet<Position> = HashSet::new();
    let mut result: Vec<Antinode> = Vec::new();

    for antinode in antinodes.into_iter() {
        println!("{:?}", antinode);
        if seen_positions.insert(antinode.position.clone()) {
            result.push(antinode);
        }
    }


    println!("\nResult: {}\n", result.len());
}

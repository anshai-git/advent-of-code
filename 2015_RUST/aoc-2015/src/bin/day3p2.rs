use std::{
    collections::{HashSet},
    fs::{self},
};

use aoc_2015::lib::utils::{parse_filename};

fn main() {
    let file: String = parse_filename()
        .and_then(|filename| fs::read_to_string(&filename).ok())
        .expect("Failed to read file");

    let mut santa_position: (isize, isize) = (0, 0);
    let mut robo_position: (isize, isize) = (0, 0);

    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    visited.insert(santa_position);

    let mut turn: bool = true;
    for step in file.chars() {
        println!("Santa: {:?}", santa_position);
        println!("Robo: {:?}", robo_position);
        println!("-------------------------");

        let pos: &mut(isize, isize) = if turn { &mut santa_position } else { &mut robo_position };

        match step {
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            _ => {
                println!("Invalid character");
            }
        }


        turn = !turn;
        visited.insert(*pos);
    }

    println!("Distinct houses: {}", visited.len());
}

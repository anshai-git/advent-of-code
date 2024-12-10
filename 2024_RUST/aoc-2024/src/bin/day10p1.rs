use std::{
    collections::HashSet, fs::File, io::{BufRead, BufReader}
};

use aoc_2024::lib::util::{open_file, parse_filename};

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .unwrap();

    let map: Vec<Vec<Option<u32>>> = BufReader::new(file)
        .lines()
        .flatten()
        .map(|line| line.chars().map(|c| c.to_digit(10)).collect())
        .collect();

    print_map(&map);

    let trailheads: Vec<(isize, isize)> = find_trailheads(&map);
    println!("Trailheads: {:?}", trailheads);

    let mut sum: u32 = 0;
    for (tindex, trailhead) in trailheads.iter().enumerate() {
        println!("Solving for trailhead {} of {} :: {:?}", tindex + 1, trailheads.len(), trailhead);
        let score: isize = traverse_map(&map, trailhead);
        println!("Trailhead score: {}", score);
        sum += score as u32;
    }
    println!("SUM :: {}", sum);
}

fn traverse_map(map: &Vec<Vec<Option<u32>>>, trailhead: &(isize, isize)) -> isize {
    let mut end_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut first_step: bool = true;

    fn step(map: &Vec<Vec<Option<u32>>>, current_position: (isize, isize), previous_position: (isize, isize), end_positions: &mut HashSet<(isize, isize)>, visited: &mut HashSet<(isize, isize)>, first_step: &mut bool) {
        // println!("{:?}", current_position);
        if current_position.0 < 0 || current_position.1 < 0 || current_position.0 >= map.len() as isize || current_position.1 >= map[0].len() as isize {
            return;
        }

        if !*first_step {
            if let (Some(a), Some(b)) = (map[current_position.0 as usize][current_position.1 as usize], map[previous_position.0 as usize][previous_position.1 as usize]) {
                if a < b || a.abs_diff(b) != 1 {
                    return;
                }
            }
        } else {
            *first_step = false;
        }

        if let Some(n) = map[current_position.0 as usize][current_position.1 as usize] {
            if n == 9 {
                end_positions.insert(current_position);
                return;
            }
        } else {
            return;
        }

        if visited.contains(&current_position) { return; }
        visited.insert(current_position);

        step(map, (current_position.0 + 1, current_position.1), current_position, end_positions, visited, first_step); // Down
        step(map, (current_position.0, current_position.1 - 1), current_position, end_positions, visited, first_step); // Left
        step(map, (current_position.0 - 1, current_position.1), current_position, end_positions, visited, first_step); // Up
        step(map, (current_position.0, current_position.1 + 1), current_position, end_positions, visited, first_step); // Right
    }

    step(map, *trailhead, *trailhead, &mut end_positions, &mut visited, &mut first_step);

    end_positions.len() as isize
}

fn find_trailheads(map: &Vec<Vec<Option<u32>>>) -> Vec<(isize, isize)> {
    let mut trailheads: Vec<(isize, isize)> = Vec::new();
    for (row_index, row) in map.iter().enumerate() {
        for(col_index, col) in row.iter().enumerate() {
            if let Some(n) = col {
                if *n == 0 {
                    trailheads.push((row_index as isize, col_index as isize));
                }
            }
        }
    }
    trailheads
}

fn print_map(map: &Vec<Vec<Option<u32>>>) {
    for row in map {
        for col in row {
            if let Some(n) = col {
                print!("{}", n);
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

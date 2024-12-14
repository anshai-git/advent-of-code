use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader}, process::exit,
};

use aoc_2024::lib::util::{open_file, parse_filename};
use image::RgbImage;

const max_x: isize = 101;
const max_y: isize = 103;

#[derive(Debug)]
struct Robot {
    /*           X      Y  */
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn new(position: (isize, isize), velocity: (isize, isize)) -> Self {
        Self { position, velocity }
    }

    fn advance(&mut self) {
        let pos_x = self.position.0 + self.velocity.0;
        let pos_y = self.position.1 + self.velocity.1;
        // println!("{} :: {}", pos_x, pos_y);
        self.position.0 = match pos_x {
            ..=-1 => max_x + pos_x,
            v if v >= max_x => pos_x - max_x,
            _ => pos_x,
        };
        self.position.1 = match pos_y {
            ..=-1 => max_y + pos_y,
            v if v >= max_y => pos_y - max_y,
            _ => pos_y,
        };
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .unwrap();

    let mut robots: Vec<Robot> = BufReader::new(file)
        .lines()
        .flatten()
        .map(|line| {
            let position: (isize, isize) = parse_values(line.split(' ').nth(0).unwrap());
            let velocity: (isize, isize) = parse_values(line.split(' ').nth(1).unwrap());
            Robot::new(position, velocity)
        })
        .collect::<Vec<Robot>>();

    let mut highest_frequency = (0, 0);
    for iteration in 1..10001 {
        for robot in robots.iter_mut() {
            robot.advance();
        }
        let mut count_by_position: HashMap<(isize, isize), usize> = HashMap::new();
        for robot in &robots {
            count_by_position
                .entry(robot.position)
                .and_modify(|it| *it += 1)
                .or_insert(1);
        }
        let frequency = calculate_frequency(&count_by_position);
        if highest_frequency.0 < frequency {
            highest_frequency = (frequency, iteration);
        }
        // println!("Iteration :: {} | Frequency :: {}", iteration, frequency);
    }

    println!("Highest frequency: {:?}", highest_frequency);
}

fn parse_values(raw: &str) -> (isize, isize) {
    Some(raw.split('=').nth(1).unwrap().split(','))
        .and_then(|mut parts| {
            Some((
                parts.nth(0).unwrap().parse::<isize>().unwrap(),
                parts.nth(0).unwrap().parse::<isize>().unwrap(),
            ))
        })
        .unwrap()
}

fn calculate_frequency(robot_count_by_position: &HashMap<(isize, isize), usize>) -> usize {
    let mut frequency: usize = 0;

    for (pos, count) in robot_count_by_position {
        if *count > 1 {
            frequency += count;
        }
        if let Some(_) = robot_count_by_position.get(&(pos.0 + 1, pos.1)) {
            frequency += 1;
        }
        if let Some(_) = robot_count_by_position.get(&(pos.0 - 1, pos.1)) {
            frequency += 1;
        }
        if let Some(_) = robot_count_by_position.get(&(pos.0, pos.1 + 1)) {
            frequency += 1;
        }
        if let Some(_) = robot_count_by_position.get(&(pos.0, pos.1 - 1)) {
            frequency += 1;
        }
    }

    frequency
}

fn print_map(robots: &Vec<Robot>) {
    for row in 0..max_y {
        for col in 0..max_x {
            if robots
                .iter()
                .any(|r| r.position.0 == col && r.position.1 == row)
            {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn draw_pixel(robots: &Vec<Robot>, image: &mut RgbImage) {
    for row in 0..max_y {
        for col in 0..max_x {
            *image.get_pixel_mut(col as u32, row as u32) = if robots
                .iter()
                .any(|r| r.position.0 == col && r.position.1 == row)
            {
                image::Rgb([255, 255, 255])
            } else {
                image::Rgb([0, 0, 0])
            };
        }
    }
}

fn pause() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}

/*
    224264664 :: too low
    229069152
    232824384 :: too high
*/

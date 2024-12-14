use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader}, process::exit,
};

use aoc_2024::lib::util::{open_file, parse_filename};
use image::{Rgb, RgbImage};

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

    for iteration in 0..10000 {
        for robot in robots.iter_mut() {
            robot.advance();
        }
        // println!("-----------------------------");
        // for robot in &robots {
        //     println!("{:?}", robot);
        // }

        let mut image: RgbImage = image::ImageBuffer::new(max_x as u32, max_y as u32);
        draw_pixel(&robots, &mut image);

        match image.save(format!("trees/tree_{}.png", iteration)) {
            Ok(_) => {
                println!("Saved Image {}", iteration);
            },
            Err(_) => {
                println!("Failed to Save Image {}", iteration);
                exit(1);
            }
        }

        // println!("ITERATION: {}", iteration);
        // print_map(&robots);
        // pause();
    }
    let mut count_by_position: HashMap<(isize, isize), usize> = HashMap::new();
    for robot in robots {
        count_by_position
            .entry(robot.position)
            .and_modify(|it| *it += 1)
            .or_insert(1);
    }

    let mut q1: u64 = 0;
    let mut q2: u64 = 0;
    let mut q3: u64 = 0;
    let mut q4: u64 = 0;
    for (k, v) in count_by_position {
        // println!("\n\n{:?} :: {}", k, v);
        if k.0 >= 0 && k.0 < (max_x / 2) && k.1 >= 0 && k.1 < (max_y / 2) {
            q1 += v as u64;
        }
        if k.0 > (max_x / 2) && k.0 < max_x && k.1 >= 0 && k.1 < (max_y / 2) {
            q2 += v as u64;
        }
        if k.0 >= 0 && k.0 < (max_x / 2) && k.1 > (max_y / 2) && k.1 < max_y {
            q3 += v as u64;
        }
        if k.0 > (max_x / 2) && k.0 < max_x && k.1 > (max_y / 2) && k.1 < max_y {
            q4 += v as u64;
        }
        // println!("Q1 :: {} | Q2 :: {} | Q3 :: {} | Q4 :: {}", q1, q2, q3, q4);
    }

    println!("PRODUCT: {}", q1 * q2 * q3 * q4);
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

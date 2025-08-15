use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::{BufRead, BufReader},
};

use aoc_2015::lib::utils::{open_file, parse_filename};

#[derive(Debug)]
enum Operation {
    Toggle,
    On,
    Off,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl From<String> for Point {
    fn from(value: String) -> Self {
        let mut parts = value.trim().split(",");
        let x = parts.nth(0).unwrap().parse::<usize>().unwrap();
        let y = parts.nth(0).unwrap().parse::<usize>().unwrap();
        Self { x, y }
    }
}

#[derive(Debug)]
struct Instruction {
    from: Point,
    to: Point,
    operation: Operation,
}

impl Instruction {
    pub fn apply(&self, grid: &mut [[Light; 1000]; 1000]) {
        for y in self.from.y..=self.to.y {
            for x in self.from.x..=self.to.x {
                match self.operation {
                    Operation::On => grid[y][x].turn_on(),
                    Operation::Off => grid[y][x].turn_off(),
                    Operation::Toggle => grid[y][x].toggle(),
                }
            }
        }
    }
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        let mut operation: Operation = Operation::Toggle;

        println!("Value: {:?}", value);

        let mut left = "";
        if let Some(str) = value.strip_prefix("turn on ") {
            operation = Operation::On;
            left = str;
        }

        if let Some(str) = value.strip_prefix("turn off ") {
            operation = Operation::Off;
            left = str;
        }

        if let Some(str) = value.strip_prefix("toggle ") {
            operation = Operation::Toggle;
            left = str;
        }

        println!("Value: {:?}", left);

        let mut parts = left.split(" through ");
        let from = Point::from(parts.nth(0).unwrap().to_string());
        let to = Point::from(parts.nth(0).unwrap().to_string());

        let instruction = Self {
            from,
            to,
            operation,
        };

        println!("{:?}", instruction);

        instruction
    }
}

#[derive(Copy, Clone, Debug)]
struct Light {
    pub brightness: u32,
}

impl Light {
    fn new() -> Self {
        Light { brightness: 0 }
    }

    pub fn toggle(&mut self) -> () {
        self.brightness += 2;
    }

    pub fn turn_on(&mut self) -> () {
        self.brightness += 1;
    }

    pub fn turn_off(&mut self) -> () {
        if self.brightness >= 1 {
            self.brightness -= 1;
        }
    }

    pub fn brightness(&self) -> u32 {
        self.brightness
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|filename| open_file(&filename))
        .expect("Failed to read file");

    let mut grid: [[Light; 1000]; 1000] = [[Light::new(); 1000]; 1000];

    let instructions = BufReader::new(file)
        .lines()
        .flatten()
        .map(|l| Instruction::from(l))
        .collect::<Vec<Instruction>>();

    for inst in instructions.iter() {
        inst.apply(&mut grid);
    }

    let mut count = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            count += grid[i][j].brightness();
        }
    }

    println!("Count: {}", count);
}

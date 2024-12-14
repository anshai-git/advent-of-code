use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use aoc_2024::lib::util::{open_file, parse_filename};

#[derive(Debug)]
struct Machine {
    a: (u128, u128),
    b: (u128, u128),
    prize: (u128, u128),
}

impl Machine {
    fn solve(&self) -> u128 {
        let mut prices: Vec<u128> = Vec::new();
        for lim in 2..100 {
            for i in 1..lim {
                let a_divider = (i * self.a.0) + ((lim - i) * self.b.0);
                let b_divider = (i * self.a.1) + ((lim - i) * self.b.1);
                if (self.prize.0 % a_divider == 0) &&
                   (self.prize.1 % b_divider == 0) {
                    let a = (self.prize.0 / a_divider) * i;
                    let b = (self.prize.1 / b_divider) * (lim - i);

                    println!("({} % (({} * {}) + ({} * {})) == 0) && ({} % (({} * {}) + ({} * {})) == 0)", self.prize.0, i, self.a.0, lim - i, self.b.0, self.prize.1, i, self.a.1, lim - i, self.b.1);
                    let price = (a * 3) + b;
                    prices.push(price);
                }
            }
        }
        *(prices.iter().min().or(Some(&0)).unwrap())
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .unwrap();

    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();

    let machines_raw: Vec<Vec<String>> = lines
        .split(|s| s.is_empty())
        .map(|group| group.to_vec())
        .collect();

    let machines: Vec<Machine> = machines_raw.iter().map(|machine_raw| {
        let a: (u128, u128) = parse_part(&machine_raw[0], '+', 0);
        let b: (u128, u128) = parse_part(&machine_raw[1], '+', 0);
        let prize: (u128, u128) = parse_part(&machine_raw[2], '=', 0); // PADDING for p2
        Machine { a, b, prize }
    }).collect();

    let sum: u128 = machines.iter().map(|m| m.solve()).sum();
    println!("{:?}", sum);
}

fn parse_part(button_string: &String, splitter: char, padding: u128) -> (u128, u128) {
    Some(
        button_string
            .split(':')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|part| {
                part.split(splitter)
                    .nth(1)
                    .unwrap()
                    .parse::<u128>()
                    .ok()
                    .unwrap() + padding
            })
            .to_owned()
            .collect::<Vec<u128>>(),
    )
    .map(|v| (v[0], v[1]))
    .unwrap()
}

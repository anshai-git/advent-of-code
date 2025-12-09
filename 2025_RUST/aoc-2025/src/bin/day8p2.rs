use std::{
    fs::File,
    io::{BufRead, BufReader},
};

extern crate aoc_2025;
use aoc_2025::lib::utils::{open_file, parse_filename};

#[derive(Debug)]
struct BoxPosition {
    x: f64,
    y: f64,
    z: f64,
}

impl BoxPosition {
    fn from(raw: String) -> Self {
        let parts: Vec<&str> = raw.splitn(3, ',').collect();
        Self::new(
            parts[0].parse::<f64>().unwrap(),
            parts[1].parse::<f64>().unwrap(),
            parts[2].parse::<f64>().unwrap(),
        )
    }

    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn distance(&self, other: &BoxPosition) -> f64 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0) + (self.z - other.z).powf(2.0))
            .powf(0.5)
    }
}

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .expect("Filed to open file");

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .flatten()
        .map(|l| l.to_string())
        .collect();

    let boxes: Vec<BoxPosition> = lines.into_iter().map(|l| BoxPosition::from(l)).collect();
    let mut circuits: Vec<Vec<&BoxPosition>> = boxes.iter().map(|b| vec![b]).collect();

    let mut boxes_by_diff: Vec<(f64, (&BoxPosition, &BoxPosition))> = Vec::new();
    for i in 0..boxes.len() {
        for j in i..boxes.len() {
            if i == j {
                continue;
            }

            let item: (f64, (&BoxPosition, &BoxPosition)) =
                (boxes[i].distance(&boxes[j]), (&boxes[i], &boxes[j]));

            boxes_by_diff.push(item);
        }
    }

    boxes_by_diff.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    for item in boxes_by_diff.iter() {
        let index_a = circuits
            .iter()
            .position(|it| {
                it.iter()
                    .any(|b| b.x == item.1 .0.x && b.y == item.1 .0.y && b.z == item.1 .0.z)
            })
            .unwrap();

        let index_b = circuits
            .iter()
            .position(|it| {
                it.iter()
                    .any(|b| b.x == item.1 .1.x && b.y == item.1 .1.y && b.z == item.1 .1.z)
            })
            .unwrap();

        if index_b == index_a {
            continue;
        }

        for i in 0..circuits[index_b].len() {
            let b: &BoxPosition = circuits[index_b].get(i).unwrap();
            circuits[index_a].push(b);
        }
        circuits[index_b] = vec![];

        println!(
            "MULTI LEN COUNT: {} :: {:?} :: {}",
            circuits.iter().filter(|it| it.len() > 0).count(),
            item,
            (item.1 .0.x) * (item.1 .1.x)
        );
    }
}

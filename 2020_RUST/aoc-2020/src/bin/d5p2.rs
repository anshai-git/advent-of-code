use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use aoc_2020::lib::util::get_file;

fn main() {
    let file: File = get_file().unwrap();

    let paths: Vec<String> = BufReader::new(&file).lines().flatten().collect();
    let mut seats: Vec<usize> = Vec::new();

    for path in paths {
        let mut start = 0;
        let mut end = 127;

        for c in path.chars().take(7) {
            let mid = start + (end - start) / 2;
            match c {
                'B' => start = mid + 1,
                'F' => end = mid,
                _ => {}
            }
        }
        let row = start;

        start = 0;
        end = 7;
        for c in path.chars().skip(7).take(3) {
            let mid = start + (end - start) / 2;
            match c {
                'R' => start = mid + 1,
                'L' => end = mid,
                _ => {}
            }
        }
        let col = start;

        let seat_id = row * 8 + col;
        seats.push(seat_id);
    }
    seats.sort();
    println!("{:?}", seats);

    for i in 0..seats.len() - 2 {
        if (seats.get(i+1).unwrap() - seats.get(i).unwrap()) != 1 {
           println!("RESULT: {}", (seats.get(i+1).unwrap() - 1));
           break;
        }
    }
}

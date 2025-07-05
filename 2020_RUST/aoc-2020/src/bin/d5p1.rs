use std::{fs::File, io::{BufRead, BufReader}};

use aoc_2020::lib::util::get_file;

fn main() {
    let file: File = get_file().unwrap();

    let paths: Vec<String> = BufReader::new(&file).lines().flatten().collect();
    let mut max = 0;

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

            println!("char: {} start: {} end {}", c, start, end);
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

            println!("char: {} start: {} end {}", c, start, end);
        }
        let col = start;

        let seat_id = row * 8 + col;

        if max < seat_id {
            max = seat_id;
        }

        println!(">>> path: {}, row: {}, col: {} :: id {}", path, row, col, seat_id);
    }

    println!("SEAT ID: {}", max);


}
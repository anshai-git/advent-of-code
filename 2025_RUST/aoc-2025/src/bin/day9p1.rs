use std::{
    fs::File,
    io::{BufRead, BufReader},
};

extern crate aoc_2025;
use aoc_2025::lib::utils::{open_file, parse_filename};

fn main() {
    let file: File = parse_filename()
        .and_then(|ref filename| open_file(filename))
        .expect("Failed to open file");

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .flatten()
        .map(|l| l.to_string())
        .collect();

    let red_tiles: Vec<(usize, usize)> = lines
        .iter()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect();

    let mut max: usize = 0;
    for i in 0..red_tiles.len() {
        for j in i..red_tiles.len() {
            if i == j {
                continue;
            }
            
         let tile_a = red_tiles[i];
         let tile_b = red_tiles[j];

         let start_x = tile_a.0.min(tile_b.0);
         let start_y = tile_a.1.min(tile_b.1);

         let end_x = tile_a.0.max(tile_b.0);
         let end_y = tile_a.1.max(tile_b.1);

         let size = ((end_x - start_x) + 1) * ((end_y - start_y) + 1);
         println!("TILES {:?} :: {:?} = SIZE: {}", tile_a, tile_b, size);

         if size > max {
             max = size;
         }
        }
    }

    println!("MAX: {}", max);
}

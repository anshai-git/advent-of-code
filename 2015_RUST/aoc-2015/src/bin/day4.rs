use std::fs;

use aoc_2015::lib::utils::parse_filename;

fn main() {
    let file: String = parse_filename()
        .and_then(|filename| fs::read_to_string(&filename).ok())
        .expect("Failed to read file");

    let mut num: u64 = 0;

    let mut hash: String = String::new();
    while !hash.starts_with("000000") {
        num += 1;
        let text = format!("{}{}", file.clone().trim(), num);
        hash = format!("{:x}", md5::compute(&text));
        println!("{:?} :: {:?} :: {:?}", hash, text, num);
    }

    println!("{:?} ", num);
}

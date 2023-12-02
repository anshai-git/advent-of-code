use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn main() {
    let program: String = env::args().next().unwrap();
    let file_path: String = match env::args().count() {
        1 => {
            println!("usage: {:?} <input file>", program);
            process::exit(1);
        }
        2 => env::args().nth(1).unwrap(),
        _ => {
            eprintln!("Invalid arguments.");
            println!("usage: {:?} <input file>", program);
            process::exit(1);
        }
    };

    let input: BufReader<File> = match File::open(&file_path) {
        Ok(content) => BufReader::new(content),
        Err(_) => {
            eprintln!("Failed to read file {:?}.", file_path);
            process::exit(1);
        }
    };

    let mut result: u64 = 0;
    let mut rgb: (u64, u64, u64) = (0, 0, 0);
    for game in input.lines().filter_map(|l| l.ok()) {
        let rounds: Vec<String> = game
            .split(':').map(String::from).nth(1).unwrap()
            .split(';').map(String::from).collect();

        for round in rounds {
            let cubes: Vec<String> = round.trim().split(',').map(String::from).collect();
            for cube in cubes {
                let data: Vec<String> = cube.trim().split(' ').map(String::from).collect();
                let count = data.get(0).unwrap().parse::<u64>().unwrap();
                match data.get(1).unwrap().chars().nth(0).unwrap() {
                    'r' => if count > rgb.0 { rgb.0 = count; }
                    'g' => if count > rgb.1 { rgb.1 = count; }
                    'b' => if count > rgb.2 { rgb.2 = count; }
                    _ => {}
                }
            }
        }
        result += rgb.0 * rgb.1 * rgb.2;
        rgb = (0, 0, 0);
    }

    println!("{:?}", result);
}

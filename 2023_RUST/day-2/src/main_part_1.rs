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

    let mut result: u16 = 0;
    let mut game_index: u16 = 1;
    for game in input.lines().filter_map(|l| l.ok()) {
        let rounds: Vec<String> = game
            .split(':').map(String::from).nth(1).unwrap()
            .split(';').map(String::from).collect();
        result += game_index;
        'rounds_loop: for round in rounds {
            let cubes: Vec<String> = round.trim().split(',').map(String::from).collect();
            for cube in cubes {
                let data: Vec<String> = cube.trim().split(' ').map(String::from).collect();
                let count = data.get(0).unwrap().parse::<u8>().unwrap();
                match data.get(1).unwrap().chars().nth(0).unwrap() {
                    'r' => if count > 12 { result -= game_index; break 'rounds_loop; }
                    'g' => if count > 13 { result -= game_index; break 'rounds_loop; }
                    'b' => if count > 14 { result -= game_index; break 'rounds_loop; }
                    _ => {}
                }
            }
        }
        game_index += 1;
    }

    println!("{:?}", result);
}

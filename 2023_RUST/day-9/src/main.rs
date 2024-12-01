use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    process::exit, ops::Sub,
};

fn main() {
    let program: String = get_program_name();
    let filename: String = get_filename(program);
    let file: File = read_file(filename);

    let reader: BufReader<File> = BufReader::new(file);
    let mut result: i32 = 0;

    for line in reader.lines().flatten() {
        let mut history: Vec<i32> = line
            .split(' ')
            .map(|e| e.parse::<i32>())
            .flatten()
            .collect();

        let mut current_history: Vec<Vec<i32>> = Vec::new();
        current_history.push(history.clone());

        let mut current = history.clone();
        while current.iter().any(|e| e != &0) {
            let mut new_vec = Vec::new();
            for (i, e) in current.iter().enumerate().skip(1) {
                new_vec.push(e.sub(current.get(i-1).unwrap()) )
            }
            current = new_vec;
            current_history.push(current.clone());
        }

        let mut current_value: i32 = 0;

        for h in current_history.clone().into_iter().rev() {
            current_value = h.first().unwrap() - current_value;
        }

        history.insert(0, current_value);
        result += current_value;
    }

    println!("RESULT: {}", result);
}

fn read_file(path: String) -> File {
    match File::open(&path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to open file {path}");
            exit(1);
        }
    }
}

fn get_filename(program: String) -> String {
    match args().count() {
        2 => args().nth(1).unwrap(),
        _ => {
            eprintln!("Invalid number of arguments");
            print_usage(program);
            exit(1);
        }
    }
}

fn print_usage(program: String) -> () {
    println!("Usage: {program} <filename>");
}

fn get_program_name() -> String {
    match args().next() {
        Some(name) => name,
        None => {
            eprintln!("Program name missing.");
            exit(1);
        }
    }
}

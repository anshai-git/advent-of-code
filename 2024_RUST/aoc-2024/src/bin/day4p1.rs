use std::{env::args, fs::File, io::{self, BufRead, BufReader}, process::exit};

fn main() {
    let program: String = match args().next() {
        Some(name) => name,
        None => {
            eprintln!("Program name missing.");
            exit(1);
        }
    };

    let filename: String = match args().nth(1) {
        Some(filename) => filename,
        None => {
            println!("Usage: {program} <filename>");
            exit(1);
        }
    };

    let file: File = match File::open(&filename) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to open file {filename}");
            exit(1);
        }
    };

    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .map(|line| line.map(|l| l.chars().collect()))
        .collect::<Result<Vec<Vec<char>>, io::Error>>()
        .unwrap();

    let mut xmas_count: u64 = 0;
    for (line_index, line) in lines.iter().enumerate() {
        for (char_index, c) in line.iter().enumerate() {
            if *c == 'X' {
                if try_right(line_index, char_index, &lines) {
                    xmas_count += 1;
                }

                if try_left(line_index, char_index, &lines) {
                    xmas_count += 1;
                }

                if try_up(line_index, char_index, &lines) {
                    xmas_count += 1;
                }

                if try_down(line_index, char_index, &lines) {
                    xmas_count += 1;
                }

                if try_down_right(line_index, char_index, &lines) {
                    xmas_count += 1;
                }

                if try_down_left(line_index, char_index, &lines) {
                    xmas_count += 1;
                }

                if try_up_right(line_index, char_index, &lines) {
                    xmas_count += 1;
                }

                if try_up_left(line_index, char_index, &lines) {
                    xmas_count += 1;
                }
            }
        }
    }

    println!("Xmas Count: {:?}", xmas_count);

}

fn try_right(line_index: usize, char_index: usize, lines: &Vec<Vec<char>>) -> bool {
    if char_index > lines[0].len() - 4 { return false; }
    lines[line_index][char_index + 1] == 'M' &&
    lines[line_index][char_index + 2] == 'A' &&
    lines[line_index][char_index + 3] == 'S'
}

fn try_left(line_index: usize, char_index: usize, lines: &Vec<Vec<char>>) -> bool {
    if char_index < 3 { return false; }
    lines[line_index][char_index - 1] == 'M' &&
    lines[line_index][char_index - 2] == 'A' &&
    lines[line_index][char_index - 3] == 'S'
}


fn try_up(line_index: usize, char_index: usize, lines: &Vec<Vec<char>>) -> bool {
    if line_index < 3 { return false; }
    lines[line_index - 1][char_index] == 'M' &&
    lines[line_index - 2][char_index] == 'A' &&
    lines[line_index - 3][char_index] == 'S'
}

fn try_down(line_index: usize, char_index: usize, lines: &Vec<Vec<char>>) -> bool {
    if line_index > lines.len() - 4 { return false; }
    lines[line_index + 1][char_index] == 'M' &&
    lines[line_index + 2][char_index] == 'A' &&
    lines[line_index + 3][char_index] == 'S'
}

fn try_down_right(line_index: usize, char_index: usize, lines: &Vec<Vec<char>>) -> bool {
    if line_index > lines.len() - 4 ||
       char_index > lines[0].len() - 4 { return false; }

    lines[line_index + 1][char_index + 1] == 'M' &&
    lines[line_index + 2][char_index + 2] == 'A' &&
    lines[line_index + 3][char_index + 3] == 'S'
}

fn try_down_left(line_index: usize, char_index: usize, lines: &Vec<Vec<char>>) -> bool {
    if line_index > lines.len() - 4 ||
       char_index < 3 { return false; }

    lines[line_index + 1][char_index - 1] == 'M' &&
    lines[line_index + 2][char_index - 2] == 'A' &&
    lines[line_index + 3][char_index - 3] == 'S'
}

fn try_up_right(line_index: usize, char_index: usize, lines: &Vec<Vec<char>>) -> bool {
    if line_index < 3 ||
       char_index > lines[0].len() - 4  { return false; }

    lines[line_index - 1][char_index + 1] == 'M' &&
    lines[line_index - 2][char_index + 2] == 'A' &&
    lines[line_index - 3][char_index + 3] == 'S'
}

fn try_up_left(line_index: usize, char_index: usize, lines: &Vec<Vec<char>>) -> bool {
    if line_index < 3 ||
       char_index < 3  { return false; }

    lines[line_index - 1][char_index - 1] == 'M' &&
    lines[line_index - 2][char_index - 2] == 'A' &&
    lines[line_index - 3][char_index - 3] == 'S'
}

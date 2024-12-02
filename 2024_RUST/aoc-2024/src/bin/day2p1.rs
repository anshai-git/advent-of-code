use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

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

    let reader: BufReader<File> = BufReader::new(file);
    let mut safe_report_count: usize = 0;

    for line in reader.lines().flatten() {
        let report: Vec<isize> = line
            .split_whitespace()
            .filter_map(|it| it.parse::<isize>().ok())
            .collect();

        let is_increasing: bool = &report[0] < &report[1];
        let mut is_safe: bool = true;

        for window in report.windows(2) {
            if let [current, next] = window {
                let diff = (current - next).abs();

                if (diff < 1 || diff > 3)
                    || (is_increasing && current >= next)
                    || (!is_increasing && current <= next)
                {
                    is_safe = false;
                    break;
                }
            }
        }

        if is_safe {
            safe_report_count += 1;
        }
    }

    println!("Result: {:?}", safe_report_count);
}

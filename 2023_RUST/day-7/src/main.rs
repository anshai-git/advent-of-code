use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

#[derive(Debug)]
struct HandData {
    cards: String,
    bid: u32,
}

#[derive(Debug)]
enum HAND {
    FIVE_OF_A_KIND { data: HandData },
    FOUR_OF_A_KIND { data: HandData },
    FULL_HOUSE { data: HandData },
    THREE_OF_A_KIND { data: HandData },
    TWO_PAIR { data: HandData },
    ONE_PAIR { data: HandData },
    HIGH_CARD { data: HandData },
}

impl HAND {
    fn new(from: String, bid: u32) -> Self {
        let mut count_by_char: HashMap<char, u8> = HashMap::new();
        for c in from.chars() {
            let count = count_by_char.entry(c).or_insert(0);
            *count += 1;
        }

        // Need to sort (in decreasing order?) the hashmap by the values in order for the following matching to work properly
        for (c, count) in &count_by_char {
            match count {
                5 => { return HAND::FIVE_OF_A_KIND { data: HandData { cards: from, bid } } },
                4 => { return HAND::FOUR_OF_A_KIND { data: HandData { cards: from, bid } } },
                3 => { 
                    if count_by_char.values().any(|&v| v == 2) {

                    }
                },
                2 => { return HAND::FOUR_OF_A_KIND { data: HandData { cards: from, bid } } },
            }
        }

        HAND::TWO_PAIR { data: () }
    }

    fn strength(&self) -> u8 {
        match self {
            HAND::FIVE_OF_A_KIND { data } => 1,
            HAND::FOUR_OF_A_KIND { data } => 2,
            HAND::FULL_HOUSE { data } => 3,
            HAND::THREE_OF_A_KIND { data } => 4,
            HAND::TWO_PAIR { data } => 5,
            HAND::ONE_PAIR { data } => 6,
            HAND::HIGH_CARD { data } => 7,
        }
    }
}

fn main() {
    let program: String = args().next().unwrap();
    let filename: String = match args().count() {
        2 => args().nth(1).unwrap(),
        _ => {
            eprintln!("Invalid number of arguments.");
            print_usage(program);
            exit(1);
        }
    };

    let file: File = match File::open(&filename) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read file: <{filename}>");
            print_usage(program);
            exit(1);
        }
    };

    let reader: BufReader<File> = BufReader::new(file);

    let mut hands: Vec<HAND> = Vec::new();
    for line in reader.lines() {
        let line_str: String = line.unwrap().to_string();

        let cards: String = line_str.split(' ').nth(0).unwrap().to_string();
        let bid: u32 = line_str.split(' ').nth(1).unwrap().parse::<u32>().unwrap();

        // let hand: HAND =
        // hands.push(hand);
    }

    println!("{:?}", hands);
}

fn print_usage(program: String) -> () {
    println!("Usage: {program} <filename>");
}

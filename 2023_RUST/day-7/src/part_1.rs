use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

#[derive(Debug, Hash, Clone)]
enum Card {
    CARD_A,
    CARD_K,
    CARD_Q,
    CARD_J,
    CARD_T,
    CARD_9,
    CARD_8,
    CARD_7,
    CARD_6,
    CARD_5,
    CARD_4,
    CARD_3,
    CARD_2,
}

impl Card {
    fn new(c: char) -> Self {
        match c {
            'A' => Card::CARD_A,
            'K' => Card::CARD_K,
            'Q' => Card::CARD_Q,
            'J' => Card::CARD_J,
            'T' => Card::CARD_T,
            '9' => Card::CARD_9,
            '8' => Card::CARD_8,
            '7' => Card::CARD_7,
            '6' => Card::CARD_6,
            '5' => Card::CARD_5,
            '4' => Card::CARD_4,
            '3' => Card::CARD_3,
            _ => Card::CARD_2,
        }
    }

    fn strength(&self) -> u8 {
        match self {
            Card::CARD_A => 13,
            Card::CARD_K => 12,
            Card::CARD_Q => 11,
            Card::CARD_J => 10,
            Card::CARD_T => 9,
            Card::CARD_9 => 8,
            Card::CARD_8 => 7,
            Card::CARD_7 => 6,
            Card::CARD_6 => 5,
            Card::CARD_5 => 4,
            Card::CARD_4 => 3,
            Card::CARD_3 => 2,
            Card::CARD_2 => 1,
        }
    }
}

impl Eq for Card {}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.strength() == other.strength()
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.strength().cmp(&other.strength()))
    }
}

#[derive(Debug)]
struct HandData {
    cards: Vec<Card>,
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
    fn new(cards: Vec<Card>, bid: u32) -> Self {
        let mut count_by_card: HashMap<Card, u8> = HashMap::new();
        for card in &cards {
            let count = count_by_card.entry(card.clone()).or_insert(0);
            *count += 1;
        }

        let data: HandData = HandData { cards, bid };
        let max_count: &u8 = count_by_card.values().max().unwrap();

        return match *max_count {
            5 => HAND::FIVE_OF_A_KIND { data },
            4 => HAND::FOUR_OF_A_KIND { data },
            3 => {
                if count_by_card.len() == 2 {
                    HAND::FULL_HOUSE { data }
                } else {
                    HAND::THREE_OF_A_KIND { data }
                }
            }
            2 => {
                if count_by_card.len() == 3 {
                    HAND::TWO_PAIR { data }
                } else {
                    HAND::ONE_PAIR { data }
                }
            }
            _ => HAND::HIGH_CARD { data },
        };
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

    fn data(&self) -> &HandData {
        match self {
            HAND::FIVE_OF_A_KIND { data }
            | HAND::FOUR_OF_A_KIND { data }
            | HAND::FULL_HOUSE { data }
            | HAND::THREE_OF_A_KIND { data }
            | HAND::TWO_PAIR { data }
            | HAND::ONE_PAIR { data }
            | HAND::HIGH_CARD { data } => data,
        }
    }
}

impl Ord for HAND {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.strength() == other.strength() {
            for (a, b) in self.data().cards.iter().zip(&other.data().cards) {
                if a != b {
                    return b.cmp(&a);
                }
            }
        }

        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for HAND {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.strength() == other.strength() {
            for (a, b) in self.data().cards.iter().zip(&other.data().cards) {
                if a != b {
                    return Some(b.cmp(&a));
                }
            }
        }

        Some(self.strength().cmp(&other.strength()))
    }
}

impl Eq for HAND {}

impl PartialEq for HAND {
    fn eq(&self, other: &Self) -> bool {
        self.strength() == other.strength()
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

        let cards: Vec<Card> = line_str.split(' ').nth(0).unwrap().to_string().chars().map(Card::new).collect();
        let bid: u32 = line_str.split(' ').nth(1).unwrap().parse::<u32>().unwrap();

        let hand: HAND = HAND::new(cards, bid);
        hands.push(hand);
    }

    hands.sort();
    hands.reverse();

    let mut result: u32 = 0;
    for (index, item) in hands.iter().enumerate() {
        let rank: u32 = (index + 1) as u32;
        result += item.data().bid * rank;
        println!("HAND: {:?} | RANK: {}", item, rank);
    }

    println!("\nRESULT: {:?}", result);
}

fn print_usage(program: String) -> () {
    println!("Usage: {program} <filename>");
}

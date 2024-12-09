use std::{fmt::format, fs::read_to_string};

use aoc_2024::lib::util::parse_filename;

#[derive(Debug)]
enum ReadMode {
    EmptySpace,
    File,
}

#[derive(Debug, Clone)]
struct Space {
    value: Vec<String>,
}

impl Space {
    fn with_size(size: u32) -> Self {
        Self {
            value: vec![String::from("."); size as usize],
        }
    }
    fn available_space(&self) -> usize {
        self.value.iter().filter(|it| *it == ".").count()
    }
    fn fill(&mut self, len: usize, index: usize) {
        // let mut chars: Vec<String> = self.value.chars().collect();
        println!(
            "Fill step 1  chars :: {:?}, len :: {}, index: {}",
            self.value, len, index
        );
        let start = self.value.iter().position(|it| *it == ".").unwrap();
        println!(
            "Start fill at {} end fill at {} :: index {}",
            start, len, index
        );
        for i in start..start + len {
            if let Some(c) = self.value.get_mut(i as usize) {
                if *c == "." {
                    *c = index.to_string();
                }
            }
        }
        println!("Fill step last: {:?}", self.value);
        // self.value = s.into_iter().collect();
    }

}

#[derive(Debug, Clone)]
enum MemoryBlock {
    SpaceBlock(Space),
    FileBlock(u32, isize, bool),
}

impl MemoryBlock {
    fn file(length: u32, index: isize, moved: bool) -> Self {
        MemoryBlock::FileBlock(length, index, moved)
    }
    fn space(length: u32) -> Self {
        MemoryBlock::SpaceBlock(Space::with_size(length))
    }
}

impl ReadMode {
    fn next(&mut self) {
        *self = match self {
            ReadMode::EmptySpace => ReadMode::File,
            ReadMode::File => ReadMode::EmptySpace,
        };
    }
}

fn main() {
    let map: String = parse_filename()
        .and_then(|name| read_to_string(&name).ok())
        .unwrap();

    let mut read_mode: ReadMode = ReadMode::File;
    let mut file_index: isize = -1;

    let mut memory_blocks: Vec<MemoryBlock> = Vec::new();

    for c in map.chars().filter(|it| !it.is_whitespace()) {
        let digit: u32 = c.to_digit(10).unwrap();
        println!("Read [{digit}] as {:?}", read_mode);
        match read_mode {
            ReadMode::File => {
                file_index += 1;
                memory_blocks.push(MemoryBlock::file(digit, file_index, false));
            }
            ReadMode::EmptySpace => {
                memory_blocks.push(MemoryBlock::space(digit));
            }
        };
        read_mode.next();
    }

    let mut moved_indexes: Vec<usize> = Vec::new();

    let mut block_index = memory_blocks.len() - 1;
    while block_index > 0 {
        // print_memory(&memory_blocks);
        if let MemoryBlock::FileBlock(length, index, _) = memory_blocks[block_index] {
            for sb_index in 0..block_index {
                if let MemoryBlock::SpaceBlock(space) = &mut memory_blocks[sb_index] {
                    if space.available_space() >= length as usize {
                        moved_indexes.push(block_index);
                        space.fill(length as usize, index as usize);
                        break;
                    }
                }
            }
        }
        block_index -= 1;
    }

    for idx in &moved_indexes {
        if let MemoryBlock::FileBlock(l, _, _) = memory_blocks[*idx] {
            memory_blocks[*idx] = MemoryBlock::space(l);
        }
    }

    let mut res: Vec<Option<u64>> = Vec::new();
    for block in &memory_blocks {
        match block {
            MemoryBlock::SpaceBlock(space) => {
                space
                    .value
                    .iter()
                    .filter(|it| *it != ".")
                    .map(|it| it.parse::<u64>())
                    .flatten()
                    .for_each(|it| res.push(Some(it)));
                for _ in 0..space.available_space() {
                    res.push(None);
                }
            }
            MemoryBlock::FileBlock(length, index, toched) => {
                for _ in 0..*length {
                    res.push(Some(*index as u64));
                }
            }
        };

    }

    let mut sum: u64 = 0;
    for (index, nopt) in res.iter().enumerate() {
        if let Some(n) = nopt {
            println!("SUM [{}] += index [{}] * n [{}]", sum, index, n);
            sum += index as u64 * n;
        }
    }

    println!("\nSUM: {}", sum);
}

fn print_memory(memory_blocks: &Vec<MemoryBlock>) {
    println!("\n\n");
    for block in memory_blocks {
        let text = match block {
            MemoryBlock::SpaceBlock(space) => space.value.join(""),
            MemoryBlock::FileBlock(length, index, toched) => {
                vec![index.to_string(); *length as usize].join("")
            }
        };
        print!("{}", text);
    }
    println!("\n\n");
}

/*

    84572976416     :: low
    6289564621904   :: high
    6289564433984   :: correct
*/

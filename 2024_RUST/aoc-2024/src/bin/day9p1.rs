use std::fs::read_to_string;

use aoc_2024::lib::util::parse_filename;

#[derive(Debug)]
enum ReadMode {
    EmptySpace,
    File,
}

#[derive(Debug, Clone)]
enum MemoryBlock {
    Space(u32),
    Value(u32, isize)
}

impl MemoryBlock {
    fn value(value: u32, index: isize) -> Self {
       MemoryBlock::Value(value, index)
    }
    fn space(length: u32) -> Self {
       MemoryBlock::Space(length)
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
        match read_mode {
            ReadMode::File => {
                file_index += 1;
                memory_blocks.push(MemoryBlock::value(digit, file_index));
            },
            ReadMode::EmptySpace => {
                memory_blocks.push(MemoryBlock::space(digit));
            }
        };
        read_mode.next();
    }

    for block in &memory_blocks {
        let text = match block {
            MemoryBlock::Space(length) => vec!['.'; *length as usize].iter().collect(),
            MemoryBlock::Value(val, index) => vec![index.to_string(); *val as usize].join("")
        };
    }

    let mut blocks_copy = memory_blocks.clone();

    let mut result: String = String::new();
    let mut result2: Vec<u32> = Vec::new();
    let mut rightmost_index = memory_blocks.len() - 1;
    for (index, block) in memory_blocks.iter_mut().enumerate() {
        if index >= rightmost_index {
            loop {
                if let MemoryBlock::Value(ref mut val, index) = &mut blocks_copy[rightmost_index] {
                    result.push_str(index.to_string().as_str());
                    result2.push(*index as u32);
                    *val -= 1;

                    if *val <= 0 {
                        break;
                    }
                } else {
                    break;
                }
            }
            break;
        }

        match block {
            MemoryBlock::Value(value, index) => {
                while *value > 0 {
                    result.push_str(index.to_string().as_str());
                    result2.push(*index as u32);
                    *value -= 1;
                }
            }
            MemoryBlock::Space(length) => {
                while *length > 0 {
                    if index >= rightmost_index {
                        break;
                    }
                    while !matches!(blocks_copy[rightmost_index], MemoryBlock::Value(_, _)) {
                        rightmost_index -= 1;
                    }

                    if let MemoryBlock::Value(ref mut val, index) = &mut blocks_copy[rightmost_index] {
                        result.push_str(index.to_string().as_str());
                        result2.push(*index as u32);
                        *val -= 1;

                        if *val == 0 {
                            *length -= 1;
                            rightmost_index -= 1;
                            continue;
                        }
                    }

                    *length -= 1;
                }
            }
        }
    }

    let mut sum: u64 = 0;
    for (index, el) in result2.iter().enumerate() {
        sum += *el as u64 * index as u64;
    }

    println!("Sum: {}", sum);
}

/*
    022111222
    022111222

    0..111....22222
    0..111....22222

    00...111...2...333.44.5555.6666.777.888899
    00...111...2...333.44.5555.6666.777.888899

    0099811188827773336446555566 :: correct
    0099811188827773336446555566
*/

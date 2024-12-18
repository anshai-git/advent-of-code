use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::BitXor,
    process::exit,
};

#[derive(Debug)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl OpCode {
    fn from_num(num: usize) -> Self {
        match num {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => {
                eprintln!("Invalid OPCODE :: {}", num);
                exit(1);
            }
        }
    }
}

struct Computer {
    reg_a: isize,
    reg_b: isize,
    reg_c: isize,
    program: Vec<usize>,
    ip: isize,
}

impl Computer {
    fn new() -> Self {
        Self {
            ip: 0,
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            program: vec![],
        }
    }

    fn load_program(&mut self, regs: (isize, isize, isize), program: Vec<usize>) {
        self.reg_a = regs.0;
        self.reg_b = regs.1;
        self.reg_c = regs.2;
        self.program = program;
        self.ip = 0;
    }

    fn run_program(&mut self) -> String {
        let mut output: Vec<usize> = Vec::new();

        while !self.is_at_end() {
            let instruction: OpCode = OpCode::from_num(self.advance());

            match instruction {
                OpCode::Adv => {
                    let left: isize = self.reg_a;
                    let literal: usize = self.advance();
                    let right = isize::pow(2, self.get_combo_operand(&literal) as u32);
                    let res: isize = left / right;
                    self.reg_a = res;
                }
                OpCode::Bxl => {
                    let left: isize = self.reg_b;
                    let right: usize = self.advance();
                    let res = left.bitxor(right as isize);
                    self.reg_b = res;
                }
                OpCode::Bst => {
                    let literal: usize = self.advance();
                    let res: isize = self.get_combo_operand(&literal) % 8;
                    self.reg_b = res;
                }
                OpCode::Jnz if self.reg_a != 0 => {
                    self.ip = self.advance() as isize;
                }
                OpCode::Bxc => {
                    self.advance(); // For legacy reasons this instruction reads an operand but it ignores it.
                    let left = self.reg_b;
                    let right = self.reg_c;
                    let res = left.bitxor(right);
                    self.reg_b = res;
                }
                OpCode::Out => {
                    let literal: usize = self.advance();
                    let res = self.get_combo_operand(&literal) % 8;
                    output.push(res as usize);
                }
                OpCode::Bdv => {
                    let left: isize = self.reg_a;
                    let literal: usize = self.advance();
                    let right = isize::pow(2, self.get_combo_operand(&literal) as u32);
                    let res: isize = left / right;
                    self.reg_b = res;
                }
                OpCode::Cdv => {
                    let left: isize = self.reg_a;
                    let literal: usize = self.advance();
                    let right = isize::pow(2, self.get_combo_operand(&literal) as u32);
                    let res: isize = left / right;
                    self.reg_c = res;
                }
                _ => {}
            }

            // println!("{:?}", instruction);
        }

        output
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn get_combo_operand(&mut self, operand: &usize) -> isize {
        match operand {
            0 | 1 | 2 | 3 => *operand as isize,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => {
                eprintln!("Invalid Instruction, no combo operand for: {:?}", operand);
                exit(1);
            }
        }
    }

    fn advance(&mut self) -> usize {
        if !self.is_at_end() {
            let res = *self.program.get(self.ip as usize).unwrap();
            self.ip += 1;
            return res;
        }
        0
    }

    fn is_at_end(&mut self) -> bool {
        self.ip as usize >= self.program.len()
    }
}

use aoc_2024::lib::util::{open_file, parse_filename};

fn main() {
    let file: File = parse_filename().and_then(|ref f| open_file(f)).unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();

    let mut parts = lines.split(|l| l.is_empty());
    let mut regs: (isize, isize, isize) = parse_regs(parts.nth(0));
    let program = parse_program(parts.nth(0));

    let target = program
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",");

    println!("\nPROGRAM :: {:?}", program);
    println!("\nTARGET PROGRAM :: {:?}", target);

    let mut computer: Computer = Computer::new();

    regs.0 = 0;

    computer.load_program(regs, program.clone());

    loop {
        let output = computer.run_program();
        println!("REG A :: {} | TARGET: {} :: {}", regs.0, target, output);
        // pause();
        if target == output {
            break;
        }
        regs.0 += 1;
        computer.load_program(regs, program.clone());
    }

    // println!("OUTPUT: {}", output);
}

fn parse_program(source: Option<&[String]>) -> Vec<usize> {
    source
        .and_then(|lines| lines.get(0))
        .and_then(|line| line.split(':').nth(1))
        .map(|part| {
            part.split(',')
                .filter_map(|el| el.trim().parse::<usize>().ok())
                .collect()
        })
        .unwrap_or_else(Vec::new)
}

fn parse_regs(source: Option<&[String]>) -> (isize, isize, isize) {
    source
        .and_then(|p| {
            Some((
                parse_reg_val(p.get(0).unwrap()),
                parse_reg_val(p.get(1).unwrap()),
                parse_reg_val(p.get(2).unwrap()),
            ))
        })
        .unwrap()
}

fn parse_reg_val(source: &String) -> isize {
    source
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse::<isize>()
        .unwrap()
}

fn pause() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}

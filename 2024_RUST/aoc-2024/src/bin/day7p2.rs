use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    process::exit,
};

use aoc_2024::lib::util::{open_file, parse_filename};

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Mul,
    Cat,
}

fn main() {
    let file = parse_filename().and_then(|filename| open_file(&filename));
    if file.is_none() {
        exit(1);
    }

    let reader = BufReader::new(file.unwrap());

    let mut calibrations: HashMap<usize, Vec<usize>> = HashMap::new();
    for line in reader.lines().flatten() {
        let mut parts = line.split(':');
        let num = parts.nth(0).unwrap().parse::<usize>().unwrap();
        let values: Vec<usize> = parts
            .nth(0)
            .unwrap()
            .split(' ')
            .map(|it| it.parse::<usize>().ok())
            .flatten()
            .collect();
        calibrations.insert(num, values);
    }

    let mut result: usize = 0;
    let mut cal_index = 0;
    let mut operator_permutations_by_length: HashMap<usize, Vec<Vec<Operator>>> = HashMap::new();
    for (k, v) in &calibrations {
        let operator_count = v.len() - 1;
        let mut operators = operator_permutations_by_length.entry(operator_count)
            .or_insert_with(|| generate_operator_lists(vec![Operator::Add, Operator::Mul, Operator::Cat], operator_count))
            .clone();

        let operators_len = operators.len();
        for (index, o) in operators.iter_mut().enumerate() {
            let mut sum: usize = 0;
            let mut operator = Operator::Add;

            println!(
                "Solving for calibration {} of {} permutation {} of {}",
                cal_index,
                calibrations.len(),
                index,
                operators_len
            );

            for el in v {
                match operator {
                    Operator::Add => sum += el,
                    Operator::Mul => sum *= el,
                    Operator::Cat => {
                        let left = sum.to_string();
                        let right = el.to_string();
                        let concatted = format!("{}{}", left, right);
                        // println!("CAT >> {} || {} = {}", left, right, concatted);

                        sum = concatted.parse::<usize>().unwrap();
                    }
                }
                if let Some(op) = o.pop() {
                    operator = op;
                }
            }
            if sum == *k {
                result += sum;
                break;
            }
            // println!("{} :: {:?} :: {}", k, v, sum);
        }
        cal_index += 1;
    }

    println!("\nResult: {}\n", result);
}

fn generate_operator_lists(operators: Vec<Operator>, positions: usize) -> Vec<Vec<Operator>> {
    let mut result: Vec<Vec<Operator>> = Vec::new();

    fn backtrack(
        current: &mut Vec<Operator>,
        positions: usize,
        operators: &Vec<Operator>,
        result: &mut Vec<Vec<Operator>>,
    ) {
        if current.len() == positions {
            result.push(current.clone());
            return;
        }

        for operator in operators {
            current.push(operator.clone());
            backtrack(current, positions, operators, result);
            current.pop();
        }
    }

    backtrack(&mut Vec::new(), positions, &operators, &mut result);

    result
}

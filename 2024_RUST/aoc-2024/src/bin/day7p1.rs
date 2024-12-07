use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
    process::exit,
};

use aoc_2024::lib::util::{open_file, parse_filename};

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Mul
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
    for (k, v) in calibrations {
        let mut operators: Vec<Vec<Operator>> = generate_operator_lists(vec![Operator::Add, Operator::Mul], v.len() - 1);
        for o in &operators {
            println!("{:?}", o);
        }

        for o in operators.iter_mut() {
            let mut sum: usize = 0;
            let mut operator = Operator::Add;
            for el in &v {
                match operator {
                    Operator::Add => { sum += el },
                    Operator::Mul => { sum *= el }
                }
                if let Some(op) = o.pop() {
                    operator = op;
                }
            }
            if sum == k {
                // result +=  v.iter().sum::<usize>();
                result +=  sum;
                break;
            }
            println!("{} :: {:?} :: {}", k, v, sum);
        }
    }

    println!("\nResult: {}\n", result);

}

fn generate_operator_lists(operators: Vec<Operator>, positions: usize) -> Vec<Vec<Operator>> {
    let mut result: Vec<Vec<Operator>> = Vec::new();

    fn backtrack(current: &mut Vec<Operator>, positions: usize, operators: &Vec<Operator>, result: &mut Vec<Vec<Operator>>) {
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

use core::panic;
use std::{collections::VecDeque, fs};

fn main() {
    let filename = "inputs/q18_input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    let results: Vec<i64> = contents
        .lines()
        .map(|x| apply_bootleg_shunting_yard(x))
        .collect();

    println!("{:?}", results.iter().sum::<i64>());
}

fn apply_bootleg_shunting_yard(expression: &str) -> i64 {
    let symbols = expression
        .replace(" ", "")
        .chars()
        .into_iter()
        .collect::<Vec<char>>();

    let mut output: VecDeque<char> = VecDeque::new(); //queue
    let mut operator: VecDeque<char> = VecDeque::new(); //stack
    for symbol in symbols {
        match symbol {
            '0'..='9' => {
                output.push_back(symbol);
            }
            '*' | '+' | '/' | '-' => {
                // precedence and associativity don't matter
                while !operator.is_empty() && *operator.back().unwrap() != '(' {
                    output.push_back(operator.pop_back().unwrap());
                }
                operator.push_back(symbol);
            }
            '(' => operator.push_back(symbol),
            ')' => {
                while *operator.back().unwrap() != '(' {
                    output.push_back(operator.pop_back().unwrap())
                }
                if *operator.back().unwrap() == '(' {
                    operator.pop_back();
                }
            }
            _ => {}
        }
    }
    while !operator.is_empty() {
        output.push_back(operator.pop_back().unwrap())
    }

    let mut curr_output = output
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>();

    while curr_output.len() > 1 {
        let mut output_index: usize = 0;
        for (i, item) in curr_output.iter_mut().enumerate() {
            match item.as_str() {
                "*" | "+" | "-" => {
                    output_index = i;
                    break;
                }
                _ => {}
            }
        }
        let n1: i64 = curr_output[output_index - 2].clone().parse().unwrap();
        let n2: i64 = curr_output[output_index - 1].clone().parse().unwrap();
        let op = curr_output[output_index].clone();

        let new_val = match op.as_str() {
            "*" => n1 * n2,
            "-" => n1 - n2,
            "+" => n1 + n2,
            _ => panic!("Something broke"),
        };

        for _ in 0..=2 {
            curr_output.remove(output_index - 2);
        }
        curr_output.insert(output_index - 2, new_val.to_string());
    }
    curr_output[0].parse::<i64>().unwrap()
}

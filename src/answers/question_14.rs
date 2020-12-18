use std::{collections::HashMap, fs};

fn main() {
    let filename = "inputs/q14_input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    let mut curr_mask: Vec<char> = vec![];
    let mut curr_mask_copy;
    let mut memory: HashMap<i64, i64> = HashMap::new();

    for line in contents.lines() {
        if line.contains("mask") {
            curr_mask = line
                .split(" = ")
                .nth(1)
                .unwrap()
                .chars()
                .collect::<Vec<char>>();
        } else {
            curr_mask_copy = curr_mask.clone();
            let value = line.split(" = ").nth(1).unwrap().parse::<i64>().unwrap();
            let bin_value = format!("{:b}", value);
            let location = line
                .split("mem[")
                .nth(1)
                .unwrap()
                .split(']').next()
                .unwrap()
                .parse::<i64>()
                .unwrap();

            for (i, curr_char) in bin_value.chars().rev().enumerate() {
                match curr_mask_copy[curr_mask.len() - 1 - i] {
                    '1' => curr_mask_copy[curr_mask.len() - 1 - i] = '1',
                    'X' => curr_mask_copy[curr_mask.len() - 1 - i] = curr_char,
                    '0' => curr_mask_copy[curr_mask.len() - 1 - i] = '0',
                    _ => {}
                }
            }

            let integer_value = curr_mask_copy
                .iter()
                .map(|&x| if x == 'X' { '0' } else { x })
                .collect::<String>();
            let result = i64::from_str_radix(&integer_value, 2).unwrap();
            memory.insert(location, result as i64);
        }
    }

    let sum: i64 = memory.values().sum();
    println!("{:?}", sum);
}

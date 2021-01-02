use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, vec,
};
#[derive(Debug, Clone, Copy)]
struct Node {
    value: u32,
    next: usize,
}

fn main() {
    let moves = 100;
    let mut nodes = "137826495"
        .chars()
        .into_iter()
        .enumerate()
        .map(|(i, x)| Node {
            value: x.to_digit(10).unwrap(),
            next: i + 1,
        })
        .collect::<Vec<Node>>();

    let max_value: u32 = nodes.iter().max_by_key(|&x| x.value).unwrap().value.clone();
    let min_value: u32 = nodes.iter().min_by_key(|&x| x.value).unwrap().value.clone();

    let mut templast = nodes.last_mut().unwrap();
    templast.next = 0;

    let mut curr_cup_index = 0;
    for curr_move in 1..=moves {
        let nodes_snapshot = nodes.clone();
        let prev_curr_cup_index = curr_cup_index;

        println!("Turn: {:?}: ", curr_move);
        print!("Start:\t");
        print_in_order(&nodes, prev_curr_cup_index);

        let mut three_cups_index = vec![];
        for _ in 1..=4 {
            curr_cup_index = nodes_snapshot.get(curr_cup_index).unwrap().next;
            three_cups_index.push(curr_cup_index as u32);
        }
        let after_three_index = three_cups_index.pop().unwrap();

        // select destination
        let mut possible_destination_index = 0;
        let mut possible_destination_value =
            if nodes_snapshot.get(prev_curr_cup_index).unwrap().value - 1 >= min_value {
                nodes_snapshot.get(prev_curr_cup_index).unwrap().value - 1
            } else {
                max_value
            };
        let three_values: Vec<u32> = three_cups_index
            .iter()
            .map(|&x| nodes_snapshot[x as usize].value)
            .collect();

        loop {
            if three_values.contains(&possible_destination_value) {
                possible_destination_value = if possible_destination_value - 1 < min_value {
                    max_value
                } else {
                    possible_destination_value - 1
                };
            } else {
                for (i, x) in nodes_snapshot.iter().enumerate() {
                    if x.value == possible_destination_value {
                        possible_destination_index = i;
                    }
                }
                break;
            }
        }

        nodes.get_mut(prev_curr_cup_index as usize).unwrap().next = after_three_index as usize;

        // Place three cups at destination
        let temp_destination_index = nodes.get(possible_destination_index).unwrap().next;
        nodes.get_mut(possible_destination_index).unwrap().next = three_cups_index[0] as usize;

        // change last of picked up cups to destination next
        nodes.get_mut(three_cups_index[2] as usize).unwrap().next = temp_destination_index;

        print!("End:\t");
        print_in_order(&nodes, prev_curr_cup_index);
    }

    let one_position = nodes.iter().position(|x| x.value == 1).unwrap();
    println!("Remove 1 from beginning of following:");
    print_in_order(&nodes, one_position);
}

fn print_in_order(nodes: &Vec<Node>, start_index: usize) {
    let mut curr_index = start_index;
    let mut count = 0;
    loop {
        let temp = nodes.get(curr_index).unwrap();
        print!("{:?}", temp.value);
        curr_index = temp.next;

        if curr_index == start_index || count >= nodes.len() {
            break;
        }
        count += 1;
    }
    println!();
}

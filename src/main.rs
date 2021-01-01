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
    // let nodes: Vec<Node> = Vec::new();
    let mut nodes = "389125467"
        .chars()
        .into_iter()
        .enumerate()
        .map(|(i, x)| Node {
            value: x.to_digit(10).unwrap(),
            next: i + 1,
        })
        .collect::<Vec<Node>>();

    let mut templast = nodes.last_mut().unwrap();
    templast.next = 0;

    println!("{:?}", nodes);
    let moves = 10;

    let mut curr_dest = 0;
    let mut curr_cup = 0;
    let mut previous_cup: Node;
    for x in 0..moves {
        let nodes_snapshot = nodes.clone();
        println!("Turn: {:?}", x);
        let prev_curr_cup = curr_cup;
        previous_cup = nodes_snapshot.get(prev_curr_cup).unwrap().clone();

        let mut skip_start = nodes_snapshot.get(prev_curr_cup).unwrap();
        let mut skip_end = 0;

        let mut three_cups = vec![];
        for jump in 1..=3 {
            // match jump {
            //     3 => {
            //         skip_end = nodes_snapshot.get(curr_cup).unwrap().next;
            //         curr_cup = nodes_snapshot.get(skip_end).unwrap().next;
            //     }
            //     _ => {
            //         curr_cup = nodes_snapshot.get(curr_cup).unwrap().next;
            //     }
            // }
            curr_cup = nodes_snapshot.get(curr_cup).unwrap().next;
            three_cups.push(nodes_snapshot.get(curr_cup).unwrap().clone());
        }
        // let mut third_cup = curr_cup;
        println!("skip_start: {:?}, skip_end: {:?}", &skip_start, &skip_end);
        println!("prev: {:?}, curr: {:?}", &prev_curr_cup, &curr_cup);

        println!("three cups: {:?}", &three_cups);

        // select destination
        while 

        break;
    }
}

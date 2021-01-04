use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, vec,
};

fn main() {
    let filename = "inputs/q22_input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    let mut player_one_deck: VecDeque<i32> = VecDeque::new();
    let mut player_two_deck: VecDeque<i32> = VecDeque::new();
    let mut curr_player = 0;
    for line in contents.lines() {
        if line.contains("Player") {
            curr_player = line
                .split("Player ")
                .nth(1)
                .unwrap()
                .split(":")
                .nth(0)
                .unwrap()
                .parse::<i32>()
                .unwrap();

            println!("Player: {:?}", curr_player);
        } else if line != "" {
            println!("{:?}", line);
            let value = line.trim().parse::<i32>().unwrap();

            match curr_player {
                1 => {
                    player_one_deck.push_back(value);
                }
                2 => {
                    player_two_deck.push_back(value);
                }
                _ => {}
            }
        }
    }

    // println!("{:?}", player_one_deck);

    // Simulate game
    while !player_one_deck.is_empty() && !player_two_deck.is_empty() {
        let pone = player_one_deck.pop_front().unwrap();
        let ptwo = player_two_deck.pop_front().unwrap();

        if pone > ptwo {
            player_one_deck.push_back(pone);
            player_one_deck.push_back(ptwo);
        } else {
            player_two_deck.push_back(ptwo);
            player_two_deck.push_back(pone);
        }
    }

    println!("{:?}", player_one_deck);
    println!("{:?}", player_two_deck);

    let mut score = 0;
    let mut winning_deck: VecDeque<i32>;

    if player_one_deck.is_empty() {
        winning_deck = player_two_deck;
        println!("Player 2 won!");
    } else {
        winning_deck = player_one_deck;
        println!("Player 1 won!");
    }

    let mut scores = (1..=winning_deck.len())
        .map(|x| x as i32)
        .collect::<Vec<i32>>();
    scores.reverse();

    for (card_num, card_value) in winning_deck.iter().enumerate() {
        println!("{:?}", score);
        score = score + scores[card_num] * card_value;
    }
    println!("Score: {:?}", score);
}

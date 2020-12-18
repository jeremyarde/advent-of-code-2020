use std::collections::HashMap;

fn main() {
    let mut puzzle_input = vec![16, 1, 0, 18, 12, 14, 19];
    let final_turn = 30000000;

    let mut recent: HashMap<i32, Vec<usize>> = HashMap::new();

    // follow the example turn numbers
    for (i, &x) in puzzle_input.iter().enumerate() {
        recent.insert(x, vec![i + 1]);
    }
    let turn_num = recent.len() + 1;

    for turn in turn_num..=final_turn {
        let last_value = *puzzle_input.last().unwrap();
        let value_repeated = match recent.get(&last_value) {
            Some(x) => x.len() > 1,
            None => false,
        };

        // if true then we have two values in the list, we can try
        if value_repeated {
            let recent_turns = recent.get_mut(&last_value).unwrap();
            let last_two_turns = &recent_turns[recent_turns.len() - 2..recent_turns.len()];
            let earlier_turn = last_two_turns[0];
            let later_turn = last_two_turns[1];
            puzzle_input.push((later_turn - earlier_turn) as i32);
            match recent.get_mut(&((later_turn - earlier_turn) as i32)) {
                Some(x) => {
                    x.push(turn);
                }
                None => {
                    recent.insert((later_turn - earlier_turn) as i32, vec![turn]);
                }
            }
        } else {
            puzzle_input.push(0);
            match recent.get_mut(&0) {
                Some(x) => {
                    x.push(turn);
                }
                None => {
                    recent.insert(0, vec![turn]);
                }
            }
        }
    }
    println!("{:?}", puzzle_input.last().unwrap());
}

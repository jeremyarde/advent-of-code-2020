use std::{collections::HashMap, fs};

fn main() {
    let filename = "inputs/q12_input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    let mut direction = 'E';

    let mut direction_graph: HashMap<char, char> = HashMap::new();
    let mut rev_direction_graph: HashMap<char, char> = HashMap::new();

    direction_graph.insert('N', 'E');
    direction_graph.insert('E', 'S');
    direction_graph.insert('S', 'W');
    direction_graph.insert('W', 'N');

    rev_direction_graph.insert('S', 'E');
    rev_direction_graph.insert('E', 'N');
    rev_direction_graph.insert('N', 'W');
    rev_direction_graph.insert('W', 'S');

    let mut move_distance: HashMap<char, i32> = HashMap::new();
    move_distance.insert('E', 0);
    move_distance.insert('N', 0);
    move_distance.insert('S', 0);
    move_distance.insert('W', 0);

    for line in contents.lines() {
        let curr_char = line.chars().next().unwrap();
        let value = line
            .chars()
            .rev()
            .take(line.len() - 1)
            .collect::<Vec<char>>()
            .iter()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        match curr_char {
            'R' | 'L' => {
                let rotations = value / 90;
                let directed_graph = match curr_char {
                    'R' => &direction_graph,
                    'L' => &rev_direction_graph,
                    _ => panic!("Failed"),
                };
                for _ in 0..rotations {
                    direction = *directed_graph.get(&direction).unwrap();
                }
            }
            'N' | 'E' | 'W' | 'S' => match move_distance.get_mut(&curr_char) {
                Some(x) => *x += value,
                None => {
                    move_distance.insert(curr_char, value);
                }
            },
            'F' => {
                let temp = move_distance.get_mut(&direction).unwrap();
                *temp += value;
            }
            _ => {}
        }
    }

    println!(
        "{:?}",
        (move_distance.get(&'N').unwrap() - move_distance.get(&'S').unwrap()).abs()
            + (move_distance.get(&'W').unwrap() - move_distance.get(&'E').unwrap()).abs()
    )
}

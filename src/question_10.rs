use std::{collections::HashMap, fs};

fn main() {
    let filename = "inputs/q10_input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    let mut ratings = contents
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    ratings.sort_unstable();
    ratings.insert(0, 0);
    ratings.push(ratings.last().unwrap() + 3);

    let mut differences: HashMap<i32, i32> = HashMap::new();

    for (i, &x) in (&ratings[0..ratings.len() - 1]).iter().enumerate() {
        match differences.get_mut(&(ratings[i + 1] - x)) {
            Some(x) => *x += 1,
            None => {
                differences.insert(ratings[i + 1] - x, 1);
            }
        }
    }
    println!("{:?}", differences);
}

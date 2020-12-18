use std::fs;

fn main() {
    let filename = "inputs/q9_input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    let preamble_count = 25;

    let all_contents: Vec<i64> = contents
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut slice_end_index = preamble_count;

    while slice_end_index < all_contents.len() {
        let slice = &all_contents[slice_end_index - preamble_count..slice_end_index];
        let mut flatfacts = vec![];
        for (i, element) in slice.iter().enumerate() {
            for (j, inner_element) in slice.iter().enumerate() {
                if i != j {
                    flatfacts.push((element + inner_element) as i64);
                }
            }
        }
        let value_to_find = &all_contents[slice_end_index];
        if !flatfacts.contains(value_to_find) {
            println!("Did not find value: {:?}", value_to_find);
            break;
        }

        slice_end_index += 1;
    }
}

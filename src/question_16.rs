use std::fs;

fn main() {
    let filename = "inputs/q16_input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    let mut valid_tickets: Vec<i32> = vec![];
    let mut valid_ranges: Vec<(i32, i32)> = vec![];

    let ranges = contents.split("your ticket:").collect::<Vec<&str>>()[0];
    let all_ranges = ranges.split("\r\n").collect::<Vec<&str>>();
    for line in all_ranges {
        for token in line.split(" ") {
            if token.contains("-") {
                let range_start_ends = token
                    .split("-")
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>();
                valid_ranges.push((range_start_ends[0], range_start_ends[1]));
            }
        }
    }

    let nearby_tickets = contents.split("nearby tickets:").collect::<Vec<&str>>()[1];
    let nearby_ticket_nums = nearby_tickets
        .replace("\r\n", ",")
        .split(",")
        .map(|x| x.parse().unwrap_or(-1))
        .filter(|&x| x >= 0)
        .collect::<Vec<i32>>();

    for ticket in &nearby_ticket_nums {
        for valid_range in &valid_ranges {
            if (valid_range.0..=valid_range.1).contains(ticket) {
                valid_tickets.push(ticket.clone());
                break;
            }
        }
    }

    let num_errors: i32 = nearby_ticket_nums.len() as i32 - valid_tickets.len() as i32;
    let sum_errors: i32 =
        nearby_ticket_nums.iter().sum::<i32>() - valid_tickets.iter().sum::<i32>();

    println!(
        "number of errors: {:?}\nsum of error tickets: {:?}",
        num_errors, sum_errors
    );
}

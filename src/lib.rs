use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::vec;

fn question_1() {
    let filename = "inputs/q1_input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    let mut numbers: Vec<i64> = contents
        .lines()
        .into_iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    numbers.sort_unstable();

    let mut curr_start = 0;
    let mut curr_end = (numbers.len()) - 1;

    while numbers[curr_end] + numbers[curr_start] != 2020 {
        match numbers[curr_end] + numbers[curr_start] {
            x if x > 2020 => {
                curr_end -= 1;
            }
            x if x < 2020 => {
                curr_start += 1;
            }
            _ => {}
        }
    }

    println!("Results: {:?}", numbers[curr_end] * numbers[curr_start]);
}

#[derive(Debug)]
struct Password {
    min: i32,
    max: i32,
    letter: char,
    password: String,
}
fn question_2() {
    let filename = "inputs/q2_inputs.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    let passwords: Vec<Password> = contents.lines().map(|x| parse_password(x)).collect();

    let valid_pw_count: i64 = passwords.into_iter().map(is_valid).filter(|x| *x).count() as i64;

    println!("{}", valid_pw_count);
}

fn is_valid(pw: Password) -> bool {
    let letter_count = pw.password.matches(pw.letter).count() as i32;
    if pw.min <= letter_count && letter_count <= pw.max {
        return true;
    }
    false
}

fn parse_password(x: &str) -> Password {
    let items = x.split(' ');
    let mut password = Password {
        min: 0,
        max: 1,
        letter: '-',
        password: String::from("-"),
    };
    for (i, item) in items.into_iter().enumerate() {
        match i {
            0 => {
                let min_max: Vec<&str> = item.split('-').collect();
                password.min = min_max[0].parse::<i32>().unwrap();
                password.max = min_max[1].parse::<i32>().unwrap();
            }
            1 => {
                password.letter = item.chars().collect::<Vec<char>>()[0];
            }
            2 => {
                password.password = item.to_string();
            }
            _ => {}
        }
    }
    password
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

fn question_3() {
    let filename = "inputs/q3_inputs.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    let mut tree_count = 0;
    let move_directions = Pos { x: 1, y: 2 };
    let mut curr_position = Pos { x: 0, y: 0 };

    let hill: Vec<&str> = contents.lines().collect();
    let board_size = Pos {
        x: hill[0].len() as i32 - 1,
        y: hill.len() as i32 - 1,
    };

    while curr_position.y <= board_size.y {
        if let '#' = hill[curr_position.y as usize]
            .chars()
            .nth(curr_position.x as usize)
            .unwrap()
        {
            tree_count += 1;
        }

        curr_position.x += move_directions.x;
        curr_position.y += move_directions.y;

        if curr_position.x > board_size.x {
            curr_position.x = curr_position.x - board_size.x - 1;
        }
    }
    println!("Trees hit: {:?}", tree_count);
}

fn question_4() {
    let filename = "inputs/q4_input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    let pw_contents: Vec<&str> = contents.lines().collect();

    let mut joined_pws: Vec<String> = vec![];
    let mut curr_joined_pw: String = String::new();

    for x in pw_contents.into_iter() {
        if x != "" {
            let mut empty = String::new();
            empty.push_str(x);
            empty.push(' ');
            curr_joined_pw.push_str(empty.as_str());
        } else {
            joined_pws.push(curr_joined_pw.clone());
            curr_joined_pw.clear();
        }
    }
    joined_pws.push(curr_joined_pw);

    let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .map(|x| (x, false))
        .collect::<HashMap<&str, bool>>();

    let results: Vec<bool> = joined_pws
        .into_iter()
        .map(|x| q4_is_valid(x, &required_keys))
        .collect();

    println!("{:?}", results.clone().into_iter().filter(|&x| x).count());
    println!("{:?}", results.into_iter().filter(|&x| !x).count());
}

fn q4_is_valid(passport: String, required_keys: &HashMap<&str, bool>) -> bool {
    for &key in required_keys.keys() {
        if !passport.contains(key) {
            return false;
        }
    }
    true
}

#[derive(Debug)]
struct SeatRange {
    low: i32,
    high: i32,
}
fn question_5() {
    let filename = "inputs/q5_input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    let results = contents
        .lines()
        .into_iter()
        .map(|x| evaluate_seat(x))
        .map(|x| x.0 * 8 + x.1);

    println!("All finished! {:?}", results.max().unwrap());
}

fn modify_seat(mut seat_range: SeatRange, keep_high: bool) -> SeatRange {
    let range_size = seat_range.high - seat_range.low;

    if keep_high {
        seat_range.low += range_size / 2 + 1;
    } else {
        seat_range.high = range_size / 2 + seat_range.low;
    }
    seat_range
}

fn evaluate_seat(seat_id: &str) -> (i32, i32) {
    let mut seat_range = SeatRange { low: 0, high: 127 };
    let mut seat_column_range = SeatRange { low: 0, high: 7 };

    for letter in seat_id.chars().into_iter() {
        match letter {
            'F' => seat_range = modify_seat(seat_range, false),
            'B' => seat_range = modify_seat(seat_range, true),
            'L' => seat_column_range = modify_seat(seat_column_range, false),
            'R' => seat_column_range = modify_seat(seat_column_range, true),
            _ => {}
        }
    }

    (seat_range.high, seat_column_range.high)
}

fn question_6() {
    let filename = "inputs/q6_input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    let grouped_contents = contents.split("\r\n\r\n").collect::<Vec<&str>>();

    let yes_groups: Vec<String> = grouped_contents
        .into_iter()
        .map(|x| x.replace('\n', ""))
        .map(|x| x.replace('\r', ""))
        .collect();

    let yes_counts: Vec<i32> = yes_groups
        .into_iter()
        .map(|x| {
            let mut hash = HashSet::new();
            for char in x.chars().into_iter() {
                hash.insert(char);
            }
            hash.len() as i32
        })
        .collect();

    let yes_total: i32 = yes_counts.iter().sum();

    println!("{:?}", yes_total);
}

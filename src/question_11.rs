use std::fs;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

fn main() {
    let filename = "inputs/q11_input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    let mut curr_state = vec![];

    for line in contents.lines().map(String::from) {
        let mut currline = vec![];
        for char in line.chars() {
            currline.push(char);
        }
        curr_state.push(currline);
    }

    let mut curr_pos = Pos { x: 0, y: 0 };
    let max_pos = Pos {
        x: curr_state[0].len() as i32,
        y: curr_state.len() as i32,
    };

    loop {
        let mut updated_state = curr_state.clone();

        for y in (0..max_pos.y).into_iter() {
            curr_pos.y = y;
            for x in (0..max_pos.x).into_iter() {
                curr_pos.x = x;
                match updated_state[y as usize][x as usize] {
                    'L' | '#' => {
                        let occ_seats = count_occupied_seats(&curr_state, &curr_pos, &max_pos);

                        if updated_state[y as usize][x as usize] == 'L' && occ_seats == 0 {
                            updated_state[y as usize][x as usize] = '#';
                        } else if updated_state[y as usize][x as usize] == '#' && occ_seats >= 4 {
                            updated_state[y as usize][x as usize] = 'L';
                        }
                    }
                    _ => {}
                }
            }
        }

        if curr_state == updated_state {
            break;
        }

        curr_state = updated_state;
    }

    let occupied_seats = curr_state
        .into_iter()
        .flatten()
        .collect::<String>()
        .matches('#')
        .count();

    println!("{:?}", occupied_seats);
}

fn count_occupied_seats(seats: &[Vec<char>], pos: &Pos, max_pos: &Pos) -> i32 {
    let mut occupied_surrounding = 0;

    let valid_x: Vec<i32> = vec![(pos.x as i32) + 1, pos.x as i32, pos.x as i32 - 1];
    let valid_x = valid_x
        .iter()
        .filter(|&x| x < &(max_pos.x as i32) && x >= &mut 0)
        .copied()
        .collect::<Vec<i32>>();

    let valid_y: Vec<i32> = vec![(pos.y as i32) + 1, pos.y as i32, pos.y as i32 - 1];
    let valid_y = valid_y
        .iter()
        .filter(|&&x| x < (max_pos.y as i32) && x >= 0)
        .copied()
        .collect::<Vec<i32>>();

    for &y in valid_y.iter() {
        for &x in valid_x.iter() {
            if !(pos.x == x && pos.y == y) {
                if let '#' = &seats[y as usize][x as usize] {
                    occupied_surrounding += 1;
                }
            }
        }
    }

    occupied_surrounding
}

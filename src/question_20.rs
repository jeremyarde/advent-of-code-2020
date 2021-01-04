use std::{
    collections::{HashMap, HashSet},
    fs, vec,
};
#[derive(Debug)]
struct Tile {
    num: i32,
    image: Vec<String>,
}

impl Tile {
    fn rotate(self) {
        let mut new_image: Vec<String> = Vec::new();
        let mut side_value: String = String::new();
    }

    fn flip(self) {}
}

enum SIDE {
    Left,
    Right,
    Top,
    Bottom,
}

fn main() {
    let filename = "inputs/q20_input_test.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    // println!("{:?}", contents);
    let image_size = 10;
    let left_side_index = 0;
    let right_side_index = 9;

    let mut tiles: Vec<Tile> = Vec::new();
    let mut curr_tile: Vec<String> = vec![];
    let mut curr_tilenum = 0;
    for line in contents.lines() {
        // println!("{:?}", line);
        if line.contains("Tile ") {
            curr_tile.clear();
            curr_tilenum = line
                .split("Tile ")
                .nth(1)
                .unwrap()
                .replace(":", "")
                .parse::<i32>()
                .unwrap();
            println!("tilenum: {:?}", curr_tilenum);
        } else if line == "" {
            let newtile = Tile {
                num: curr_tilenum,
                image: curr_tile.clone(),
            };
            tiles.push(newtile);
        } else {
            curr_tile.push(line.to_string());
        }
    }

    for tile in &tiles {
        println!("{:?}", tile.image);
        println!("{:?}", get_side(tile, SIDE::Top));
        println!("{:?}", get_side(tile, SIDE::Right));
        println!("{:?}", get_side(tile, SIDE::Bottom));
        println!("{:?}", get_side(tile, SIDE::Left));
    }

    println!("{:?}", tiles);
}

fn get_side(tile: &Tile, side: SIDE) -> String {
    let mut side_value: String = String::new();
    match side {
        SIDE::Left | SIDE::Right => {
            let side_index = match side {
                SIDE::Right => 9,
                SIDE::Left => 0,
                _ => 0,
            };

            for line in &tile.image {
                let tempchar = line.chars().nth(side_index).unwrap();
                side_value.push(tempchar);
            }
        }
        SIDE::Top => side_value = tile.image.get(0).unwrap().clone(),
        SIDE::Bottom => side_value = tile.image.get(9).unwrap().clone(),
    }
    return side_value;
}

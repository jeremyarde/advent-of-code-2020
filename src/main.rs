use itertools::Itertools;
use std::{collections::HashSet, fs, vec};
#[derive(Debug, Clone)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
    active: bool,
}

impl Cube {
    fn nearby_active(&self, other: &Cube) -> bool {
        let coord_diffs = vec![self.x - other.x, self.y - other.y, self.z - other.z];
        let valid_diffs = vec![-1, 0, 1];
        for diff in coord_diffs {
            if !valid_diffs.contains(&diff) {
                return false;
            }
        }
        if other.active {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    // inspiration:
    // https://www.reddit.com/r/rust/comments/ker83r/advent_of_code_2020_day_17/
    // https://github.com/AxlLind/AdventOfCode2020/blob/master/src/bin/17.rs

    //     let contents = "...#...#
    // #######.
    // ....###.
    // .#..#...
    // #.#.....
    // .##.....
    // #.####..
    // #....##.";

    let contents = ".#.
..#
###";

    let dim = 3;
    let coords = (0..dim).map(|_| -1..=1).multi_cartesian_product();
    // .collect::<Vec<i8>>();
    println!("{:?}", coords);

    let mut active: HashSet<(i8, i8, i8)> = HashSet::new();

    let rounds = 6;
    // let mut curr_round = 0;

    let mut cubes: Vec<Cube> = vec![];

    for (y, line) in contents.lines().enumerate() {
        println!("{:?}", line);
        for (x, curr_char) in line.chars().enumerate() {
            // cubes.push(Cube {
            //     x: x as i32,
            //     y: y as i32,
            //     z: 0,
            //     active: if curr_char == '.' { true } else { false },
            // })
            if curr_char == '#' {
                active.insert((x as i8, y as i8, 0));
            }
        }
    }

    println!("{:?}", active);

    // println!("{:#?}", cubes.len());
    // println!("{:?}", cubes);

    for new_round in 0..rounds {
        let mut new_cubes = vec![];
        for cube in &cubes {
            let cords = vec!['x', 'y', 'z'];
            for cord in cords {
                let mut cube_one = cube.clone();
                let mut cube_two = cube.clone();
                match cord {
                    'x' => {
                        cube_one.x = cube_one.x + 1;
                        cube_two.x = cube_two.x - 1;
                    }
                    'y' => {
                        cube_one.y = cube_one.y + 1;
                        cube_two.y = cube_two.y - 1;
                    }
                    'z' => {
                        cube_one.z = cube_one.z + 1;
                        cube_two.z = cube_two.z - 1;
                    }
                    _ => {}
                }
                // deactivate the new cubes
                cube_one.active = false;
                cube_two.active = false;

                new_cubes.push(cube_one);
                new_cubes.push(cube_two);
                new_cubes.push(cube.clone());
            }
        }
        cubes = new_cubes;

        println!("Num cubes {:?}", cubes.len());
        let active_count: i32 = cubes
            .iter()
            .map(|x| if x.active { 1 } else { 0 })
            .collect::<Vec<i32>>()
            .iter()
            .sum();

        println!("{:?}", active_count);

        let mut cubes_clone = cubes.clone();
        let num_cubes = cubes.len();
        for i in 0..num_cubes {
            // let curr_cube: &mut Cube = cubes.iter_mut().nth(i).unwrap();
            let mut curr_cube = &cubes[i];
            let mut nearby_count = 0;
            for j in 0..num_cubes {
                // let inner_cube = cubes.iter_mut().nth(j).unwrap();
                // let inner_cube = cubes.iter_mut().nth(j).unwrap();
                let inner_cube = &cubes[j];
                if j != i && curr_cube.nearby_active(inner_cube) == true {
                    nearby_count += 1;
                }
            }
            if curr_cube.active == true && nearby_count == 2 || nearby_count == 3 {
                cubes_clone[i].active = true;
            } else if curr_cube.active == false && nearby_count == 3 {
                cubes_clone[i].active = true;
            }
            // println!();
        }
        cubes = cubes_clone;

        let active_count: i32 = cubes
            .iter()
            .map(|x| if x.active { 1 } else { 0 })
            .collect::<Vec<i32>>()
            .iter()
            .sum();

        println!("{:?}", active_count);

        // cycle through the cubes and modify active status based on rules
        // for (curr_cube, cube) in cubes.iter().enumerate() {

        //     let nearby_count =

        //     for curr_check in 0..cubes.len() {
        //         let neighbors = cube.nearby(curr_cube);
        //     }
        // }
    }
    let active_count: i32 = cubes
        .iter()
        .map(|x| if x.active { 1 } else { 0 })
        .collect::<Vec<i32>>()
        .iter()
        .sum();
    println!("{:?}", active_count);
    println!("{:#?}", cubes.len());
    println!("{:?}", cubes);
}

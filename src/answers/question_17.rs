use std::collections::{HashMap, HashSet};

fn main() {
    let contents = "...#...#
#######.
....###.
.#..#...
#.#.....
.##.....
#.####..
#....##.";
    let rounds = 6;

    let mut active_points: HashSet<(i8, i8, i8)> = HashSet::new();

    let neighbors_mutation: Vec<(i8, i8, i8)> = (0..3 * 3 * 3)
        .filter(|&i| i != 3 * 3 * 3 / 2)
        .map(|i| (i % 3 - 1, i / 3 % 3 - 1, i / 9 - 1))
        .collect();

    let negative_y = (contents.split('\n').count() as i8) / 2;
    for (y, line) in contents.lines().enumerate() {
        let _: i8 = (line.len() / 2) as i8;
        for (x, curr_char) in line.chars().enumerate() {
            let negative_x = (line.len() / 2) as i8;
            if curr_char == '#' {
                active_points.insert((x as i8 - negative_x, y as i8 - negative_y, 0));
            }
        }
    }

    for round in 0..rounds {
        println!("Round #{:?}, active: {:?}", round, active_points.len());
        let active = get_active(&neighbors_mutation, &active_points);
        // println!("active: {:?}", active.len());

        active_points = active;
    }
    println!("Round #{:?}, active: {:?}", rounds, active_points.len());
}

fn get_active(
    neighbors_mutation: &Vec<(i8, i8, i8)>,
    active_points: &HashSet<(i8, i8, i8)>,
) -> HashSet<(i8, i8, i8)> {
    let mut neighborhood: HashMap<(i8, i8, i8), usize> = HashMap::new();

    for &active in active_points {
        let neighbors = get_neighbors(active, &neighbors_mutation);
        for neighbor in neighbors {
            *neighborhood.entry(neighbor).or_insert(0) += 1;
        }
    }

    // Find new neighbors
    let mut new_active_points: HashSet<(i8, i8, i8)> = HashSet::new();
    for (x, num_neighbors) in neighborhood.iter() {
        if active_points.contains(x) && (*num_neighbors == 2 || *num_neighbors == 3) {
            new_active_points.insert(*x);
        } else if !active_points.contains(x) && *num_neighbors == 3 {
            new_active_points.insert(*x);
        } else {
        }
    }
    new_active_points
}

fn get_neighbors(
    current_pos: (i8, i8, i8),
    neighbors_mutation: &Vec<(i8, i8, i8)>,
) -> Vec<(i8, i8, i8)> {
    let mut neighbors: Vec<(i8, i8, i8)> = Vec::new();

    for mutation in neighbors_mutation {
        let mut mutated = current_pos;
        mutated.0 += mutation.0;
        mutated.1 += mutation.1;
        mutated.2 += mutation.2;

        neighbors.push(mutated);
    }
    neighbors
}

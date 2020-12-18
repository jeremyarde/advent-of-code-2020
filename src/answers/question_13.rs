use std::fs;

fn main() {
    let filename = "inputs/q13_input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Could not read the file")
        .lines()
        .take(2)
        .map(String::from)
        .collect::<Vec<String>>();

    let earliest_depart = &contents[0].parse::<f32>().unwrap();
    let busses = &contents[1]
        .split(',')
        .map(|x| {
            if x == "x" {
                -1.0
            } else {
                x.parse::<f32>().unwrap()
            }
        })
        .filter(|&x| x > 0.0)
        .collect::<Vec<f32>>();

    let departures: Vec<(usize, f32, f32)> = busses
        .iter()
        .map(|&x| (x, (earliest_depart / x).ceil() * x - earliest_depart))
        .enumerate()
        .map(|(i, (bus_id, value))| (i, bus_id, value))
        .collect::<Vec<(usize, f32, f32)>>();

    let mut min_wait = departures[0].2;
    let mut bus_id = departures[0].1;
    for (_, bus, value) in departures {
        if value < min_wait {
            min_wait = value;
            bus_id = bus;
        }
    }

    println!("{:?}", (bus_id * min_wait) as i32);
}

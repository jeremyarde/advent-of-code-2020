use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone)]
struct Edge {
    from: String,
    to: String,
    weight: i32,
}

fn main() {
    let filename = "inputs/q7_input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    let mut edges: Vec<Edge> = vec![];

    for line in contents.lines().into_iter() {
        let key: Vec<&str> = line.split(" bag").collect();
        let mut results: Vec<String> = key.into_iter().map(|x| get_colour(x)).collect();
        results = results
            .into_iter()
            .filter(|x| x != "no other")
            .filter(|x| x != "s.")
            .filter(|x| x != ".")
            .filter(|x| x != "contain no other")
            .collect::<Vec<String>>();

        let len_results = results.len();
        let results_clone = results.clone();
        let start_key = results[0].as_str();

        for item in results_clone.into_iter().rev().take(len_results - 1) {
            let new_edge = Edge {
                from: start_key.to_string(),
                to: item
                    .chars()
                    .rev()
                    .take(item.len() - 2)
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect(),
                weight: item.chars().take(1).collect::<String>().parse().unwrap(),
            };
            edges.push(new_edge);
        }
    }

    // Create a normal and reverse graph from the edges
    // let mut graph: HashMap<String, Vec<Edge>> = HashMap::new();
    let mut reverse_graph: HashMap<String, Vec<Edge>> = HashMap::new();
    for edge in edges {
        // let edgecopy = edge.clone();
        // println!("{:?}", edge);
        // if graph.contains_key(&edge.from) {
        //     println!("Found list, inserting");
        //     println!("{:?}", graph.get_mut(&edge.from).unwrap());
        //     graph.get_mut(&edge.from).unwrap().push(edgecopy);
        // } else {
        //     println!("Not found");
        //     graph.insert(edge.from.clone(), vec![edgecopy]);
        // }

        if reverse_graph.contains_key(&edge.to) {
            reverse_graph.get_mut(&edge.to).unwrap().push(edge);
        } else {
            reverse_graph.insert(edge.to.clone(), vec![edge]);
        }
    }

    let mut visited = HashSet::new();
    recursive_dfs(&reverse_graph, &String::from("shiny gold"), &mut visited);

    println!(
        "Number of bags that can contain \"shiny gold\": {:?}",
        visited.len() - 1
    );
}

fn get_colour(sentence_piece: &str) -> String {
    let tokens: Vec<&str> = sentence_piece.split(' ').collect();
    let new_colour: Vec<&str> = tokens.into_iter().rev().collect();
    let all_new_colours: Vec<&str> = new_colour.into_iter().take(3).rev().collect();

    all_new_colours.join(" ")
}

fn recursive_dfs(graph: &HashMap<String, Vec<Edge>>, start: &str, visited: &mut HashSet<String>) {
    let possible_edges_to_explore = graph.get(start);
    match possible_edges_to_explore {
        Some(edges_to_explore) => {
            for edge in edges_to_explore.iter() {
                // early stopping if parents have been explored already
                if !visited.contains(&edge.from) {
                    recursive_dfs(graph, &edge.from, visited);
                }
            }
            // visited all parents of this node, therefor we can mark it as visited
            visited.insert(start.to_string());
        }
        // We found no parents, so we can mark this node as visited
        None => {
            visited.insert(start.to_string());
        }
    }
}

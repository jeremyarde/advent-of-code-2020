use std::{
    collections::{HashMap, HashSet},
    fs, vec,
};

fn main() {
    let filename = "inputs/q21_input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    // let mut all_foods: HashSet<String> = HashSet::new();
    let mut allergen_sets: HashMap<String, Vec<HashSet<String>>> = HashMap::new();
    // let ingredient_allergens: HashMap<String, String> = HashMap::new();

    let mut allergen_occurances: HashMap<String, i32> = HashMap::new();

    for line in contents.lines() {
        let items = line.split(" (");
        let food_section = items
            .clone()
            .nth(0)
            .unwrap()
            .split(" ")
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();

        let allergens = items
            .clone()
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|x| x.replace(")", "").replace("contains", "").replace(" ", ""))
            .collect::<HashSet<String>>();

        for x in &allergens {
            allergen_sets
                .entry(x.to_string())
                .or_insert(vec![])
                .push(food_section.clone());
        }

        for x in &food_section {
            *allergen_occurances.entry(x.to_string()).or_insert(0) += 1;
        }
    }

    let mut final_allergens: HashMap<String, Vec<String>> = HashMap::new();
    let keys = allergen_sets.keys().clone();
    for key in keys {
        let sets = allergen_sets.get(key).unwrap().clone();
        match sets.get(0) {
            Some(_x) => {
                let newset = sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
                    acc.intersection(hs).cloned().collect()
                });
                final_allergens.entry(key.clone()).or_insert(
                    newset
                        .clone()
                        .iter()
                        .map(|x| x.clone())
                        .collect::<Vec<String>>(),
                );
            }
            None => {}
        }
    }
    // Remove ingredients that can only belong to one food
    loop {
        let mut actual_fa: HashMap<String, Vec<String>> = HashMap::new();

        for (key, foods) in &final_allergens {
            if foods.len() != 1 {
                continue;
            } else {
                for (inner_key, inner_foods) in &final_allergens {
                    if key != inner_key && foods.len() == 1 {
                        let mut tempfoods = inner_foods.clone();
                        match &inner_foods.iter().position(|x| x == &foods[0]) {
                            Some(x) => {
                                tempfoods.remove(*x);
                            }
                            None => {}
                        }
                        actual_fa
                            .entry(inner_key.clone())
                            .or_insert(tempfoods.clone());
                    } else {
                        actual_fa
                            .entry(inner_key.clone())
                            .or_insert(inner_foods.clone());
                    }
                }
            }
        }
        final_allergens = actual_fa;
        let mut loop_again = false;
        for (_, foods) in &final_allergens {
            if foods.len() > 1 {
                loop_again = true;
            }
        }
        if loop_again == false {
            break;
        }
    }

    let allergen_ingredients: Vec<&str> = final_allergens.values().map(|x| x[0].as_str()).collect();
    let mut count = 0;
    for (x, occurances) in &allergen_occurances {
        if !allergen_ingredients.contains(&x.as_str()) {
            count += occurances
        }
    }
    println!("{:?}", count);
}

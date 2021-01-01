use std::{
    collections::{HashMap, HashSet},
    fs, vec,
};

fn main() {
    let filename = "inputs/q19_input_test.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    let mut contents: Vec<String> = contents.lines().map(|x| String::from(x)).collect();

    let rules: Vec<&str> = vec![];
    let mut rulemap: HashMap<String, String> = HashMap::new();
    let mut lettermap: HashMap<String, String> = HashMap::new();
    let mut rule_contents: HashMap<String, String> = HashMap::new();
    for line in &contents {
        println!("{:?}", line);
        let mut split_rule: Vec<String> = line.split(": ").map(|x| String::from(x)).collect();
        let mut rule_num = split_rule.iter().nth(0).unwrap().clone();
        let mut rule = split_rule.iter().nth(1).unwrap();

        if rule.contains("\"") {
            lettermap.insert(
                rule_num.parse().unwrap(),
                rule.to_string().replace("\"", ""),
            );
        }
        rulemap.insert(rule_num, rule.to_string());
        // println!("{:?}, {:?}", rule_num, rule);
    }

    // loop while a digit in rule 0
    // loop {
    let bad_chars = ('0'..'9').collect::<HashSet<char>>();
    // .iter().collect::<HashSet<char>>();
    // while rulemap
    //     .get("0")
    //     .unwrap()
    //     .chars()
    //     .collect::<HashSet<char>>()
    //     .intersection(&bad_chars)
    //     .collect::<Vec<char>>()
    //     .len()
    //     > 0
    // {
    // loop {
    while rulemap.get("0").unwrap().chars().any(char::is_numeric) {
        println!("rulemap: {:?}", &rulemap);
        // println!("curr contents: {:?}", &contents);
        println!("lettermap: {:?}", &lettermap);

        let rulemap_keys = rulemap
            .keys()
            .map(|x| x.clone())
            .collect::<Vec<String>>()
            .clone();
        for key in rulemap_keys {
            // replace values in rules with letters
            for (letter_key, letter_value) in lettermap.iter_mut() {
                *rulemap.get_mut(&key).unwrap() = rulemap
                    .get_mut(&key)
                    .unwrap()
                    .replace(letter_key, letter_value);
            }

            // Go through all rulemap values, if no digits in chars,
            //we need to add it to the lettermap
            for (key, value) in &rulemap {

                // if no digits
                //  if '|', copy other value, replace with 0, replace with 1, combine with | again
            }
        }
        println!("rulemap after sub{:#?}", rulemap);
        // break;
    }

    // Check if any value contains the '0'..'9' chars,
    // and if yes, we keep in the lines, otherwise move to map
    // also keep in mind '|' and how that works

    // println!("Rule map: {:?}\nLettermap: {:?}", rulemap, lettermap);
}

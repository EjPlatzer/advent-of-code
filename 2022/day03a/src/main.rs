use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input");

    let rucksacks = input.lines();

    let repeated = rucksacks.map(get_repeated_item);

    let priorities = repeated.map(get_priority);

    let total_priority: usize = priorities.sum();

    println!("{total_priority}");
}

fn get_repeated_item(rucksack: &str) -> char {
    let (compartment1, compartment2) = rucksack.split_at(rucksack.len() / 2);

    let compartment1: HashSet<char> = compartment1.chars().collect();
    let compartment2: HashSet<char> = compartment2.chars().collect();

    compartment1
        .intersection(&compartment2)
        .nth(0)
        .expect("No repeated element found in rucksack {rucksack}")
        .clone()
}

fn get_priority(item: char) -> usize {
    let item = item as u8;

    if item > b'Z' {
        (item - b'a' + 1) as usize
    } else {
        (item - b'A' + 27) as usize
    }
}

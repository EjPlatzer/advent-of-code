use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input");

    let rucksacks = input.lines().collect::<Vec<_>>();

    let repeated = rucksacks.chunks(3).map(get_shared_item);

    let priorities = repeated.map(get_priority);

    let total_priority: usize = priorities.sum();

    println!("{total_priority}");
}

fn get_shared_item(rucksacks: &[&str]) -> char {
    let sets = rucksacks
        .iter()
        .map(|rucksack| rucksack.chars().collect::<HashSet<char>>());

    let shared_items = sets
        .reduce(|acc, set| acc.intersection(&set).cloned().collect::<HashSet<char>>())
        .expect("Found empty rucksack group");

    shared_items
        .iter()
        .last()
        .expect("Found no shared item")
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

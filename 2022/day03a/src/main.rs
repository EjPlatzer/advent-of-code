#![feature(byte_slice_trim_ascii)]
use std::collections::HashSet;

fn main() {
    let input = include_bytes!("../input.txt");

    let rucksacks = input.trim_ascii().split(|byte| byte == &b'\n');

    let repeated = rucksacks.map(get_repeated_item);

    let priorities = repeated.map(get_priority);

    let total_priority: usize = priorities.sum();

    println!("{total_priority}");
}

fn get_repeated_item(rucksack: &[u8]) -> u8 {
    let (compartment1, compartment2) = rucksack.split_at(rucksack.len() / 2);

    let compartment1: HashSet<u8> = compartment1.iter().copied().collect();
    let compartment2: HashSet<u8> = compartment2.iter().copied().collect();

    compartment1
        .intersection(&compartment2)
        .nth(0)
        .expect("No repeated element found in rucksack {rucksack}")
        .clone()
}

fn get_priority(item: u8) -> usize {
    let priority = if item > b'Z' {
        item - b'a' + 1
    } else {
        item - b'A' + 27
    };

    priority as usize
}

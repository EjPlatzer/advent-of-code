use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldn't read input.");
    let mut inventories: Vec<usize> = input
        .split("\n\n")
        .map(|inventory| {
            inventory
                .split("\n")
                .map(|food| food.parse::<usize>().unwrap_or(0))
                .sum::<usize>()
        })
        .collect();

    inventories.sort();

    let top_3_total: usize = inventories.iter().rev().take(3).sum();

    println!("Most calories is {top_3_total}")
}

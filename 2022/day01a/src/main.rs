use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldn't read input.");
    let inventories = input.split("\n\n").map(|inventory| {
        inventory
            .split("\n")
            .map(|food| food.parse::<usize>().unwrap_or(0))
            .sum::<usize>()
    });

    let most_calories = inventories.max().unwrap_or(0);

    println!("Most calories is {most_calories}")
}

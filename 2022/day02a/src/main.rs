use choice::Choice;
use round::Round;
use std::fs;

mod choice;
mod outcome;
mod round;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldn't read input.");

    let total_score: usize = input
        .lines()
        .map(read_round)
        .map(|round| round.score())
        .sum();

    println!("{total_score}");
}

fn read_round(line: &str) -> Round {
    let mut choices = line
        .split_whitespace()
        .map(|choice| choice.parse().expect("Invalid move choice"));

    let opponent_move = choices.next().expect("Couldn't read opponent move");
    let player_move = choices.next().expect("Couldn't read player move");

    Round {
        player_move,
        opponent_move,
    }
}

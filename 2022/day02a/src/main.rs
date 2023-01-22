use choice::Choice;
use round::Round;
use std::fs;

mod choice;
mod round;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldn't read input.");

    let total_score: usize = input
        .lines()
        .map(read_round)
        .map(|round| round.play())
        .sum();

    println!("{total_score}");
}

fn read_round(line: &str) -> Round {
    let mut chars = line.chars();

    let opponent_move = Choice::from_char(
        &chars
            .nth(0)
            .expect("Could not read player's move in round {line}."),
    );

    let player_move = Choice::from_char(
        &chars
            .nth(1)
            .expect("Could not read player's move in round {line}."),
    );

    Round::new(player_move, opponent_move)
}

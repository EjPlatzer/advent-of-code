use choice::Choice;
use outcome::Outcome;
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
    let mut chars = line.chars();

    let opponent_move = chars
        .nth(0)
        .expect("Could not read player's move in round {line}.");

    let opponent_move = Choice::from_char(&opponent_move);

    let player_strategy = chars
        .nth(1)
        .expect("Could not read player's move in round {line}.");

    let player_strategy = Outcome::from_char(&player_strategy);

    let player_move = Choice::for_strategy(player_strategy, &opponent_move);

    Round {
        player_move,
        opponent_move,
    }
}

use crate::outcome::Outcome;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Choice {
    pub fn from_char(opponent_move: &char) -> Choice {
        match opponent_move {
            'A' => Choice::Rock,
            'B' => Choice::Paper,
            'C' => Choice::Scissors,
            _ => panic!("Invalid opponent's move {opponent_move}"),
        }
    }

    pub fn for_strategy(strategy: Outcome, opponent_move: &Choice) -> Choice {
        match strategy {
            Outcome::Win => opponent_move.is_beaten_by(),
            Outcome::Lose => opponent_move.beats(),
            Outcome::Draw => opponent_move.clone(),
        }
    }

    pub fn beats(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    pub fn is_beaten_by(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }

    pub fn score(self) -> usize {
        self as usize
    }
}

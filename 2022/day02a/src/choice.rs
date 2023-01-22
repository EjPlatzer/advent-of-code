#[derive(Debug, PartialEq, Eq)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Choice {
    pub fn from_char(opponent_move: &char) -> Choice {
        match opponent_move {
            'A' | 'X' => Choice::Rock,
            'B' | 'Y' => Choice::Paper,
            'C' | 'Z' => Choice::Scissors,
            _ => panic!("Invalid opponent's move {opponent_move}"),
        }
    }

    pub fn beats(&self, other: &Choice) -> Outcome {
        match (self, other) {
            (Choice::Rock, Choice::Scissors)
            | (Choice::Scissors, Choice::Paper)
            | (Choice::Paper, Choice::Rock) => Outcome::Win,
            _ if self == other => Outcome::Draw,
            _ => Outcome::Lose,
        }
    }

    pub fn score(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

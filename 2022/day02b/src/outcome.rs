pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn from_char(char: &char) -> Outcome {
        match char {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Unexpected player strategy {char}"),
        }
    }

    pub fn score(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

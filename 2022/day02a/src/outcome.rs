pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn score(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

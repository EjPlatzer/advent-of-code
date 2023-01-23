use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
pub enum ParseChoiceError {
    Unknown(String),
}

impl FromStr for Choice {
    type Err = ParseChoiceError;

    fn from_str(choice: &str) -> Result<Self, Self::Err> {
        match choice {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => Err(ParseChoiceError::Unknown(String::from(
                "Unknown move choice {choice}",
            ))),
        }
    }
}

impl Choice {
    pub fn beats(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    pub fn score(self) -> usize {
        self as usize
    }
}

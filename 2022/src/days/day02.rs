use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::days::Day;

pub struct Day02;

type Instruction = (Move, char);

impl Day for Day02 {
    type Input = Vec<Instruction>;

    fn parse(_input: &str) -> IResult<&str, Self::Input> {
        let (rest, moves) = separated_list1(
            line_ending,
            separated_pair(one_of("ABC"), tag(" "), one_of("XYZ")),
        )(_input)?;

        let moves: Vec<Instruction> = moves
            .iter()
            .map(|(opponent_move_code, my_move_code)| {
                let opponent_move: Move = Move::from(opponent_move_code).expect("Invalid opponent move code. This should be impossible, because the parser only accepts valid move codes.");

                (opponent_move, *my_move_code)
            })
            .collect();

        Ok((rest, moves))
    }

    type Output1 = usize;

    fn part_1(_input: &Self::Input) -> Self::Output1 {
        _input
            .iter()
            .map(|(opponent_move, my_move_code)| {
                let my_move = Move::from(my_move_code).expect(
                    "Invalid my move code, must be eiter rock, paper, or scissors but was {other}",
                );

                Round {
                    ours: my_move,
                    theirs: *opponent_move,
                }
            })
            .map(Round::score)
            .sum()
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        _input
            .iter()
            .map(|(opponent_move, my_goal_code)| {
                let my_goal = Outcome::from(*my_goal_code).expect("Invalid my goal code");
                let my_move = Move::for_goal(my_goal, *opponent_move);

                Round {
                    ours: my_move,
                    theirs: *opponent_move,
                }
            })
            .map(Round::score)
            .sum()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub enum ParseMoveError {
    InvlaidChoice(String),
}

impl Move {
    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn from(move_code: &char) -> Result<Move, ParseMoveError> {
        match move_code {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(ParseMoveError::InvlaidChoice(String::from(
                "Unknown move choice {choice}",
            ))),
        }
    }

    fn for_goal(goal: Outcome, opponent_move: Move) -> Move {
        match goal {
            Outcome::Draw => opponent_move,
            Outcome::Loss => *Move::ALL_MOVES
                .iter()
                .find(|m| opponent_move.beats(**m))
                .expect("Some move loses to opponent's move"),
            Outcome::Win => *Move::ALL_MOVES
                .iter()
                .find(|m| m.beats(opponent_move))
                .expect("Some move wins against opponent's move"),
        }
    }

    fn beats(&self, other: Move) -> bool {
        matches!(
            (self, other),
            (Move::Rock, Move::Scissors)
                | (Move::Paper, Move::Rock)
                | (Move::Scissors, Move::Paper)
        )
    }

    fn points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

enum Outcome {
    Loss,
    Win,
    Draw,
}

impl Outcome {
    fn from(code: char) -> Result<Outcome, ()> {
        match code {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(()),
        }
    }

    fn points(self) -> usize {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

#[derive(Debug)]
struct Round {
    ours: Move,
    theirs: Move,
}

impl Round {
    fn score(self) -> usize {
        let outcome = if self.ours.beats(self.theirs) {
            Outcome::Win
        } else if self.theirs.beats(self.ours) {
            Outcome::Loss
        } else {
            Outcome::Draw
        };

        self.ours.points() + outcome.points()
    }
}

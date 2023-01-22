use crate::{choice::Outcome, Choice};

#[derive(Debug)]
pub struct Round {
    player_move: Choice,
    opponent_move: Choice,
}

impl Round {
    pub fn new(player_move: Choice, opponent_move: Choice) -> Round {
        Round {
            player_move,
            opponent_move,
        }
    }

    pub fn play(&self) -> usize {
        let outcome: usize = match self.player_move.beats(&self.opponent_move) {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        };

        outcome + self.player_move.score()
    }
}

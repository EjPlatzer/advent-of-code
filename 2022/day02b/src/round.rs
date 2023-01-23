use crate::{outcome::Outcome, Choice};

#[derive(Debug)]
pub struct Round {
    pub player_move: Choice,
    pub opponent_move: Choice,
}

impl Round {
    pub fn score(&self) -> usize {
        let player_beats = self.player_move.beats();
        let opponent_beats = self.opponent_move.beats();

        let outcome = match (player_beats, opponent_beats) {
            _ if player_beats == self.opponent_move => Outcome::Win,
            _ if opponent_beats == self.player_move => Outcome::Lose,
            _ => Outcome::Draw,
        };

        outcome.score() + self.player_move.score()
    }
}

use std::{cmp::Ordering, str::FromStr};

use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

const MAX: Set = Set {
    red: 12,
    green: 13,
    blue: 14,
};

fn compute_output(input: &str) -> usize {
    let games = input.lines().map(|game| game.parse::<Game>().unwrap());
    let valid_games = games.filter(|game| game.sets.iter().all(|set| set <= &MAX));
    valid_games.map(|game| game.id).sum()
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^Game (?<game_id>\d+): (?<game_contents>.*)$").unwrap();
        let caps = re.captures(s).unwrap();

        let id = caps
            .name("game_id")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let contents = caps.name("game_contents").unwrap().as_str();
        let sets: Vec<_> = contents
            .split("; ")
            .map(|set| set.parse().unwrap())
            .collect();

        Ok(Game { id, sets })
    }
}

#[derive(Clone, Debug, Default, Copy, PartialEq)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for Set {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Set::default();

        for cube in s.split(", ") {
            let (quantity, color) = cube
                .split_once(' ')
                .expect("Cube must have quantity and color");
            let quantity = quantity.parse::<usize>().unwrap();
            match color {
                "red" => set.red = quantity,
                "green" => set.green = quantity,
                "blue" => set.blue = quantity,
                _ => unreachable!(),
            };
        }

        Ok(set)
    }
}

impl PartialOrd for Set {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        if self.red <= other.red && self.green <= other.green && self.blue <= other.blue {
            return Some(Ordering::Less);
        }
        if self.red >= other.red && self.green >= other.green && self.blue >= other.blue {
            return Some(Ordering::Greater);
        }
        // Incomparable
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_example_input() {
        let input = include_str!("../example_input.txt");
        let output = compute_output(input);

        assert_eq!(output, 8)
    }
}

use std::str::FromStr;

use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> usize {
    let games = input.lines().map(|game| game.parse::<Game>().unwrap());
    games
        .map(|game| {
            game.sets
                .iter()
                .copied()
                .reduce(|minimum_set, current_set| minimum_set.max(&current_set))
                .expect("There must be at least one set.")
                .power()
        })
        .sum()
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^Game (?<game_id>\d+): (?<sets>.*)$").unwrap();
        let caps = re.captures(s).unwrap();

        let id = caps
            .name("game_id")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let contents = caps.name("sets").unwrap().as_str();
        let sets: Vec<_> = contents
            .split("; ")
            .map(|set| set.parse().unwrap())
            .collect();

        Ok(Game { id, sets })
    }
}

#[derive(Clone, Debug, Default, Copy)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Set {
    fn max(&self, other: &Self) -> Self {
        Set {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
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

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_example_input() {
        let input = include_str!("../example_input.txt");
        let output = compute_output(input);

        assert_eq!(output, 2286)
    }
}

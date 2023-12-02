use std::str::FromStr;

use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> usize {
    let games = input.lines().map(parse_game);
    games
        .map(|game| {
            game.sets
                .iter()
                .fold(CubeSet::default(), |minimum_set, current_set| CubeSet {
                    red: minimum_set.red.max(current_set.red),
                    green: minimum_set.green.max(current_set.green),
                    blue: minimum_set.blue.max(current_set.blue),
                })
                .power()
        })
        .sum()
    // let max = CubeSet {
    //     red: 12,
    //     green: 13,
    //     blue: 14,
    // };
    // let valid_games = games.filter(|game| game.sets.iter().all(|set| set.does_not_exceed(&max)));

    // valid_games.map(|game| game.id).sum()
}

fn parse_game(input: &str) -> Game {
    let re = Regex::new(r"^Game (?<game_id>\d+): (?<game_contents>.*)$").unwrap();
    let caps = re.captures(input).unwrap();

    let id = caps
        .name("game_id")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

    let contents = caps.name("game_contents").unwrap().as_str();
    let sets: Vec<_> = contents
        .split("; ")
        .map(|set| {
            set.split(", ")
                .map(|cube| cube.parse::<CubeDraw>().unwrap())
                .fold(CubeSet::default(), CubeSet::add_draw)
        })
        .collect();

    Game { id, sets }
}

#[derive(Debug)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<CubeSet>,
}

#[derive(Debug)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeSet {
    fn default() -> Self {
        CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn add_draw(mut self, cube: CubeDraw) -> Self {
        match cube.color {
            CubeColor::Red => self.red += cube.quantity,
            CubeColor::Green => self.green += cube.quantity,
            CubeColor::Blue => self.blue += cube.quantity,
        };
        self
    }

    fn does_not_exceed(&self, other: &CubeSet) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

struct CubeDraw {
    quantity: usize,
    color: CubeColor,
}

impl FromStr for CubeDraw {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (quantity, color) = s.split_once(' ').unwrap();
        let quantity = quantity.parse::<usize>().unwrap();
        let color = match color {
            "red" => Ok(CubeColor::Red),
            "green" => Ok(CubeColor::Green),
            "blue" => Ok(CubeColor::Blue),
            _ => Err(()),
        }?;

        Ok(CubeDraw { quantity, color })
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

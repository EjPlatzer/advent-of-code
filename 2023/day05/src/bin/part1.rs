use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> usize {
    let mut sections = input.split("\n\n");

    let mut resources = sections.next().map(parse_seeds).expect("input lists seeds");

    let maps = sections
        .map(|map| {
            let mut lines = map.lines();
            let (source, destination) = lines
                .next()
                .map(parse_header)
                .expect("map should have a valid header");

            let ranges = lines.map(parse_range).collect();

            Map {
                special_ranges: ranges,
                source,
                destination,
            }
        })
        .collect::<Vec<_>>();

    let mut step = Step::Seed;
    while step != Step::Location {
        let map = maps
            .iter()
            .find(|m| m.source == step)
            .expect("map exists for step");

        resources = resources
            .iter()
            .map(|resource| {
                map.special_ranges
                    .iter()
                    .find(|range| {
                        (range.source_start..range.source_start + range.length).contains(resource)
                    })
                    .map(|range| {
                        (*resource as isize
                            + (range.destination_start as isize - range.source_start as isize))
                            as usize
                    })
                    .unwrap_or(*resource)
            })
            .collect();

        step = map.destination;
    }

    *resources.iter().min().unwrap()
}

fn parse_seeds(input: &str) -> Vec<usize> {
    input
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse::<usize>().expect("seeds should be a number"))
        .collect::<Vec<_>>()
}

fn parse_header(input: &str) -> (Step, Step) {
    let mut kind = input.split_once(' ').unwrap().0.split('-').take(3);

    let source: Step = kind
        .next()
        .expect("map header should have a source kind")
        .parse()
        .expect("map's source kind is a valid step");

    let destination: Step = kind
        .last()
        .expect("map header should have a destination kind")
        .parse()
        .expect("map's destination kind is a valid step");

    (source, destination)
}

fn parse_range(input: &str) -> MapRange {
    let mut parts = input.split_ascii_whitespace().map(|part| {
        part.parse::<usize>()
            .expect("map should have valid numbers")
    });
    let destination_start: usize = parts.next().expect("range has a valid destination start");

    let source_start: usize = parts.next().expect("range has a valid source start");

    let length: usize = parts.next().expect("map range should have a valid length");

    MapRange {
        destination_start,
        source_start,
        length,
    }
}

struct Map {
    special_ranges: Vec<MapRange>,
    source: Step,
    destination: Step,
}

impl Map {}

#[derive(Debug)]
struct MapRange {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Step {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(Self::Seed),
            "soil" => Ok(Self::Soil),
            "fertilizer" => Ok(Self::Fertilizer),
            "water" => Ok(Self::Water),
            "light" => Ok(Self::Light),
            "temperature" => Ok(Self::Temperature),
            "humidity" => Ok(Self::Humidity),
            "location" => Ok(Self::Location),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_example_input() {
        let input = include_str!("../example_input.txt");
        let output = compute_output(input);

        assert_eq!(output, 35)
    }
}

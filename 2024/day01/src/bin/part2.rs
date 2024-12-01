use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> String {
    let mut right_location_ids = HashMap::<usize, usize>::new();

    let left_list: Vec<usize> = input
        .lines()
        .map(|line| {
            let (left, right) = line
                .split_once("   ")
                .expect("Line should have two location IDs separated by three spaces");

            let right = parse_location_id(right);
            right_location_ids
                .entry(right)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            parse_location_id(left)
        })
        .collect();

    let similarity_score: usize = left_list
        .iter()
        .map(|location_id| location_id * right_location_ids.get(location_id).unwrap_or(&0))
        .sum();

    similarity_score.to_string()
}

fn parse_location_id(location_id: &str) -> usize {
    location_id
        .parse::<usize>()
        .expect("Location ID is an unsigned integer")
}

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_example_input() {
        let input = include_str!("../example_input.txt");
        let output = compute_output(input);

        assert_eq!(output, "31")
    }
}

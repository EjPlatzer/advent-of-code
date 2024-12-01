fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> String {
    let (mut left_list, mut right_list): (Vec<usize>, Vec<usize>) =
        input.lines().map(parse_line).collect();

    left_list.sort_unstable();
    right_list.sort_unstable();

    let total_difference: usize = left_list
        .iter()
        .zip(right_list)
        .map(|(left, right)| left.abs_diff(right))
        .sum();

    total_difference.to_string()
}

fn parse_line(line: &str) -> (usize, usize) {
    let (left, right) = line
        .split_once("   ")
        .expect("Line should have two location IDs separated by three spaces");

    (
        left.parse().expect("Location ID is an unsigned integer"),
        right.parse().expect("Location ID is an unsigned integer"),
    )
}

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_example_input() {
        let input = include_str!("../example_input.txt");
        let output = compute_output(input);

        assert_eq!(output, "11")
    }
}

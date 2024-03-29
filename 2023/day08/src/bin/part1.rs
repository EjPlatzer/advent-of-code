fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> usize {
    todo!("Solve part 1")
}

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_example_input() {
        let input = include_str!("../example_input.txt");
        let output = compute_output(input);

        assert_eq!(output, 2)
    }
    #[test]
    fn test_example_input2() {
        let input = include_str!("../example_input2.txt");
        let output = compute_output(input);

        assert_eq!(output, 6)
    }
}

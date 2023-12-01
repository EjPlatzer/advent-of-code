fn main() {
    println!("Hello, world!");
}

fn compute_output(input: &str) -> String {
    String::from("todo!")
}

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_example_input() {
        let input = include_str!("exampleInput.txt");
        let output = compute_output(input);

        assert_eq!(output, "todo!")
    }
}

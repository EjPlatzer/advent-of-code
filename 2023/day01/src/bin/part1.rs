fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|&c| c >= '0' && c <= '9')
                .collect::<Vec<_>>()
        })
        .map(get_calibration_value)
        .sum()
}

fn get_calibration_value(digits: Vec<char>) -> usize {
    let mut calibration_value = String::from("");
    let first_digit = digits[0];
    let last_digit = digits
        .last()
        .expect("Every calibration value has at least one digit");

    calibration_value.push(first_digit);
    calibration_value.push(*last_digit);

    calibration_value.parse::<usize>().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_example_input() {
        let input = include_str!("../example_input.txt");
        let output = compute_output(input);

        assert_eq!(output, 142)
    }
}

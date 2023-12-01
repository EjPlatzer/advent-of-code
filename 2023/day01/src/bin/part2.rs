fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> usize {
    input
        .lines()
        .map(find_digits)
        .map(get_calibration_value)
        .sum()
}

fn find_digits(line: &str) -> Vec<char> {
    line.chars()
        .enumerate()
        .filter_map(|(i, c)| match c {
            '1' => Some('1'),
            '2' => Some('2'),
            '3' => Some('3'),
            '4' => Some('4'),
            '5' => Some('5'),
            '6' => Some('6'),
            '7' => Some('7'),
            '8' => Some('8'),
            '9' => Some('9'),
            'o' if i + 2 < line.len() && line[i..=i + 2] == *"one" => Some('1'),
            't' if i + 2 < line.len() && line[i..=i + 2] == *"two" => Some('2'),
            't' if i + 4 < line.len() && line[i..=i + 4] == *"three" => Some('3'),
            'f' if i + 3 < line.len() && line[i..=i + 3] == *"four" => Some('4'),
            'f' if i + 3 < line.len() && line[i..=i + 3] == *"five" => Some('5'),
            's' if i + 2 < line.len() && line[i..=i + 2] == *"six" => Some('6'),
            's' if i + 4 < line.len() && line[i..=i + 4] == *"seven" => Some('7'),
            'e' if i + 4 < line.len() && line[i..=i + 4] == *"eight" => Some('8'),
            'n' if i + 3 < line.len() && line[i..=i + 3] == *"nine" => Some('9'),
            _ => None,
        })
        .collect()
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
        let input = include_str!("../example_input2.txt");
        let output = compute_output(input);

        assert_eq!(output, 281)
    }
}

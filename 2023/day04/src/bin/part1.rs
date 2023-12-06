fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(": ").and_then(|(_card, numbers)| {
                let (winning_numbers, my_numbers) = numbers
                    .split_once(" | ")
                    .expect("card has winning and my numbers");

                let winning_numbers = parse_numbers(winning_numbers);
                let my_numbers = parse_numbers(my_numbers);

                let points = my_numbers.iter().fold(0 as usize, |points, number| {
                    if winning_numbers.contains(number) {
                        if points == 0 {
                            1
                        } else {
                            points * 2
                        }
                    } else {
                        points
                    }
                });

                Some(points)
            })
        })
        .sum()
}

fn parse_numbers(numbers: &str) -> Vec<usize> {
    numbers
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>().expect("number to be valid"))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_example_input() {
        let input = include_str!("../example_input.txt");
        let output = compute_output(input);

        assert_eq!(output, 13)
    }
}

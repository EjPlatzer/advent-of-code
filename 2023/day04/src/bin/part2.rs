use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> usize {
    let mut cards: HashMap<usize, usize> = HashMap::new();
    for line in input.lines() {
        let (card, numbers) = line
            .split_once(": ")
            .expect("card should have id and numbers");

        let card_id = card
            .split_ascii_whitespace()
            .last()
            .expect("Card has id")
            .parse::<usize>()
            .expect("card id is valid number");

        let card_count = cards
            .entry(card_id)
            .and_modify(|count| *count += 1)
            .or_insert(1)
            .clone();

        let (winning_numbers, my_numbers) = numbers
            .split_once(" | ")
            .expect("card has winning and my numbers");

        let winning_numbers = parse_numbers(winning_numbers);
        let my_numbers = parse_numbers(my_numbers);

        let wins = my_numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .count();

        for n in card_id..card_id + wins {
            let card_id = n + 1;
            cards
                .entry(card_id)
                .and_modify(|count| *count += card_count)
                .or_insert(card_count);
        }
    }

    cards.into_values().sum()
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

        assert_eq!(output, 30)
    }
}

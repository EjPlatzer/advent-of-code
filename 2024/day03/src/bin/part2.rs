use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> String {
    let dos: Vec<usize> = input.match_indices("do()").map(|(i, _s)| i).collect();
    let donts: Vec<usize> = input.match_indices("don't()").map(|(i, _s)| i).collect();
    let products = input.match_indices("mul(").filter_map(|(start, _)| {
        let last_do = dos.iter().filter(|do_index| do_index < &&start).last();
        let last_dont = donts
            .iter()
            .filter(|dont_index| dont_index < &&start)
            .last();

        if last_dont.is_some_and(|last_dont_index| match last_do {
            None => true,
            Some(last_do_index) => last_dont_index > last_do_index,
        }) {
            return None;
        }

        let end = input[start..].find(')');
        if let Some(end) = end {
            let end = start + end - 1;
            let start = start + 4;
            let args = &input[start..=end];
            if let Some((arg1, arg2)) = args.split_once(',') {
                let arg1 = parse_arg(arg1);
                let arg2 = parse_arg(arg2);
                if let (Some(arg1), Some(arg2)) = (arg1, arg2) {
                    return Some(arg1 as u32 * arg2 as u32);
                }
            }
        }

        None
    });

    products.sum::<u32>().to_string()
}

const ARG_DIGITS: RangeInclusive<usize> = 1..=3;

fn parse_arg(arg: &str) -> Option<u16> {
    if !ARG_DIGITS.contains(&arg.len()) {
        return None;
    }

    arg.parse::<u16>().ok()
}

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_example_input() {
        let input = include_str!("../example_input2.txt");
        let output = compute_output(input);

        assert_eq!(output, "48")
    }
}
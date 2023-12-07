fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> usize {
    let (time, distance) = input
        .split_once('\n')
        .expect("input has time and distance lines");

    let time = time
        .chars()
        .filter(|piece| piece.is_digit(10))
        .collect::<String>()
        .parse::<usize>()
        .expect("time is valid tumber");
    let distance = distance
        .chars()
        .filter(|piece| piece.is_digit(10))
        .collect::<String>()
        .parse::<usize>()
        .expect("time is valid tumber");

    let result: usize = (0..=time)
        .filter(|acceleration| {
            let movement_millis = time - acceleration;
            let distance_moved = acceleration * movement_millis;

            distance_moved > distance
        })
        .count();

    result
}

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_example_input() {
        let input = include_str!("../example_input.txt");
        let output = compute_output(input);

        assert_eq!(output, 71503)
    }
}

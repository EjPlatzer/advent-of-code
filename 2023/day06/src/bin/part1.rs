fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> usize {
    let (time, distance) = input
        .split_once('\n')
        .expect("input has time and distance lines");

    let times = time
        .split_ascii_whitespace()
        .filter_map(|piece| piece.parse::<usize>().ok());

    let distances = distance
        .split_ascii_whitespace()
        .filter_map(|piece| piece.parse::<usize>().ok());

    let result: usize = times
        .zip(distances)
        .map(|(time, distance)| {
            (0..=time)
                .filter(|acceleration| {
                    let movement_millis = time - acceleration;
                    let distance_moved = acceleration * movement_millis;

                    distance_moved > distance
                })
                .count()
        })
        .product();

    result
}

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_example_input() {
        let input = include_str!("../example_input.txt");
        let output = compute_output(input);

        assert_eq!(output, 288)
    }
}

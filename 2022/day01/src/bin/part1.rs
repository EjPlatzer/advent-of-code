fn main() {
    let input = include_str!("input1.txt");
    let output = compute(input);
    dbg!(output);
}

fn compute(input: &str) -> usize {
    let inventories = input.split("\n\n").map(|inventory| {
        inventory
            .split("\n")
            .map(|food| food.parse::<usize>().unwrap_or(0))
            .sum::<usize>()
    });

    inventories.max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let test_input = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let output = compute(test_input);

        assert_eq!(output, 24000)
    }
}

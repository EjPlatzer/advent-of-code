fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> usize {
    let schematic = input.chars().map(SchematicChar::from);
    let line_length = schematic
        .clone()
        .position(|c| c == SchematicChar::Newline)
        .unwrap()
        + 1;

    let schematic = Schematic {
        chars: schematic.collect(),
        line_length,
    };

    let mut numbers = Vec::<Number>::new();
    let mut index = 0;
    while index < schematic.chars.len() {
        if let Some(number) = Number::try_parse_at(index, &schematic.chars) {
            numbers.push(number);

            index = number.end + 2;
        } else {
            index += 1
        }
    }

    let mut gear_ratios_total = 0;

    for (index, &c) in schematic.chars.iter().enumerate() {
        let SchematicChar::Gear = c else {
            continue;
        };

        let mut neighboring_numbers = schematic
            .neighbors(index)
            .iter()
            .filter_map(|&n| n)
            .filter_map(|neighbor| {
                numbers
                    .iter()
                    .find(|n| (n.start..=n.end).contains(&neighbor))
            })
            .collect::<Vec<_>>();

        neighboring_numbers.dedup();

        if neighboring_numbers.len() == 2 {
            let first = neighboring_numbers.first().unwrap();
            let second = neighboring_numbers.last().unwrap();
            gear_ratios_total += first.value * second.value;
        }
    }

    gear_ratios_total
}

struct Schematic {
    chars: Vec<SchematicChar>,
    line_length: usize,
}

impl Schematic {
    fn neighbors(&self, index: usize) -> Vec<Option<usize>> {
        let left = index.checked_sub(1);
        let right = index.checked_add(1);
        let top_left = left.and_then(|n| n.checked_sub(self.line_length));
        let top = index.checked_sub(self.line_length);
        let top_right = right.and_then(|n| n.checked_sub(self.line_length));
        let bottom_left = left.and_then(|n| n.checked_add(self.line_length));
        let bottom = index.checked_add(self.line_length);
        let bottom_right = right.and_then(|n| n.checked_add(self.line_length));

        vec![
            top_left,
            top,
            top_right,
            left,
            right,
            bottom_left,
            bottom,
            bottom_right,
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SchematicChar {
    Number(u8),
    Symbol,
    Gear,
    Empty,
    Newline,
}

impl SchematicChar {
    fn from(c: char) -> Self {
        match c {
            '0' => SchematicChar::Number(0),
            '1' => SchematicChar::Number(1),
            '2' => SchematicChar::Number(2),
            '3' => SchematicChar::Number(3),
            '4' => SchematicChar::Number(4),
            '5' => SchematicChar::Number(5),
            '6' => SchematicChar::Number(6),
            '7' => SchematicChar::Number(7),
            '8' => SchematicChar::Number(8),
            '9' => SchematicChar::Number(9),
            '.' => SchematicChar::Empty,
            '\n' => SchematicChar::Newline,
            '*' => SchematicChar::Gear,
            _ => SchematicChar::Symbol,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Number {
    start: usize,
    end: usize,
    value: usize,
}

impl Number {
    fn try_parse_at(start: usize, chars: &[SchematicChar]) -> Option<Self> {
        let mut index = start;
        let mut value = 0;
        while let Some(SchematicChar::Number(digit)) = chars.get(index) {
            let digit = *digit as usize;
            value = value * 10 + digit;

            index += 1;
        }

        if value > 0 {
            Some(Number {
                start,
                end: index - 1,
                value,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_example_input() {
        let input = include_str!("../example_input.txt");
        let output = compute_output(input);

        assert_eq!(output, 467835)
    }
}

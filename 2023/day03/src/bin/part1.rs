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

    let mut part_number_sum = 0;
    let mut index = 0;
    while index < schematic.chars.len() {
        if let Some(number) = Number::try_parse_at(index, &schematic.chars) {
            if schematic.is_part_number(number) {
                part_number_sum += number.value;
            }

            index = number.end + 2;
        } else {
            index += 1
        }
    }

    part_number_sum
}

struct Schematic {
    chars: Vec<SchematicChar>,
    line_length: usize,
}

impl Schematic {
    fn get(&self, index: usize) -> Option<&SchematicChar> {
        self.chars.get(index)
    }

    fn symbol_at(&self, index: usize) -> bool {
        matches!(self.get(index), Some(SchematicChar::Symbol))
    }

    fn is_part_number(&self, number: Number) -> bool {
        let left = number.start.checked_sub(1).unwrap_or(number.start);
        let right = number.end.checked_add(1).unwrap_or(number.end);

        number.value > 0
            && (
                // if left neighbor is symbol
                (left < number.start && self.symbol_at(left))
                // if right neighbor is symbol
                    || (right > number.end && self.symbol_at(right))
                    // if any above/below neighbors are symbols
                    || (left..=right).any(|i| {
                        i.checked_sub(self.line_length)
                            .is_some_and(|above| self.symbol_at(above))
                            || i.checked_add(self.line_length)
                                .is_some_and(|below| self.symbol_at(below))
                    })
            )
    }
}

#[derive(Debug, PartialEq, Eq)]
enum SchematicChar {
    Number(u8),
    Symbol,
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
            _ => SchematicChar::Symbol,
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

        assert_eq!(output, 4361)
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> String {
    let safe_levels = input.lines().filter_map(|report| {
        let mut previous_state = LevelState::Unknown;
        for level in report.split_ascii_whitespace() {
            let level = level
                .parse::<usize>()
                .expect("level should be a valid integer");

            let next_state = match previous_state {
                LevelState::Unknown => Some(LevelState::Initial(level)),
                LevelState::Initial(previous_level) => {
                    if previous_level.abs_diff(level) > 3 || previous_level == level {
                        None
                    } else if previous_level > level {
                        Some(LevelState::Decreasing(level))
                    } else {
                        Some(LevelState::Increasing(level))
                    }
                }
                LevelState::Increasing(previous_level) => {
                    let valid_next_level = (previous_level + 1)..=(previous_level + 3);
                    if valid_next_level.contains(&level) {
                        Some(LevelState::Increasing(level))
                    } else {
                        None
                    }
                }
                LevelState::Decreasing(previous_level) => {
                    let valid_next_level =
                        previous_level.saturating_sub(3)..=previous_level.saturating_sub(1);
                    if valid_next_level.contains(&level) {
                        Some(LevelState::Decreasing(level))
                    } else {
                        None
                    }
                }
            };

            if let Some(level_state) = next_state {
                previous_state = level_state
            } else {
                // Difference in levels is unsafe
                return None;
            }
        }

        // Entire report was safe
        Some(())
    });

    safe_levels.count().to_string()
}

#[derive(Debug, Clone, Copy)]
enum LevelState {
    Increasing(usize),
    Decreasing(usize),
    Initial(usize),
    Unknown,
}

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_example_input() {
        let input = include_str!("../example_input.txt");
        let output = compute_output(input);

        assert_eq!(output, "2")
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

fn compute_output(input: &str) -> String {
    let input = input.trim_end();
    let width = input.find('\n').expect("word search has multiple lines");
    let len_with_newlines = input.len();
    let input = input.replace('\n', "");
    let height = len_with_newlines - input.len() + 1;

    let matrix = Matrix {
        data: input.chars().collect(),
        width,
        height,
    };

    let mut x_mases = 0;

    for row in 0isize..height as isize {
        for column in 0isize..width as isize {
            if matrix.get(row, column) == Some('A') {
                let forward_slash = (
                    matrix.get(row - 1, column - 1),
                    matrix.get(row + 1, column + 1),
                );
                let back_slash = (
                    matrix.get(row - 1, column + 1),
                    matrix.get(row + 1, column - 1),
                );

                if is_mas(forward_slash) && is_mas(back_slash) {
                    x_mases += 1;
                }
            }
        }
    }

    x_mases.to_string()
}

fn is_mas(cross: (Option<char>, Option<char>)) -> bool {
    match cross {
        (Some('M'), Some('S')) | (Some('S'), Some('M')) => true,
        _ => false,
    }
}

#[derive(Debug, Clone)]
struct Matrix {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Matrix {
    pub fn get(&self, row: isize, column: isize) -> Option<char> {
        let index = self.index_of(row, column)?;
        Some(self.data[index])
    }

    fn index_of(&self, row: isize, column: isize) -> Option<usize> {
        if row < 0 || column < 0 {
            return None;
        }

        let row = usize::try_from(row).expect("row index is valid usize");
        let column = usize::try_from(column).expect("column index is valid usize");

        if row >= self.height || column >= self.width {
            return None;
        }

        Some(self.width * row + column)
    }
}

#[cfg(test)]
mod tests {
    use crate::compute_output;

    #[test]
    fn test_example_input() {
        let input = include_str!("../example_input.txt");
        let output = compute_output(input);

        assert_eq!(output, "9")
    }
}

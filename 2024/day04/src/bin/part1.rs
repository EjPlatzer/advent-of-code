fn main() {
    let input = include_str!("../input.txt");
    let output = compute_output(input);

    dbg!(output);
}

const XMAS_LETTERS: [char; 4] = ['X', 'M', 'A', 'S'];

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

    let mut xmases = 0;

    for row in 0isize..height as isize {
        for column in 0isize..width as isize {
            if matrix.get(row, column) == Some('X') {
                for vertical_magnitude in -1i8..=1 {
                    for horizontal_magnitude in -1i8..=1 {
                        'letters: for letter_index in 1..4 {
                            let expected_letter = XMAS_LETTERS[letter_index];

                            let letter_row: isize = row as isize
                                + (letter_index as isize * vertical_magnitude as isize);
                            let letter_column: isize = column as isize
                                + (letter_index as isize * horizontal_magnitude as isize);

                            if matrix.get(letter_row, letter_column) != Some(expected_letter) {
                                break 'letters;
                            };

                            if expected_letter == 'S' {
                                xmases += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    xmases.to_string()
}

#[derive(Debug, Clone)]
struct Matrix {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Matrix {
    // pub fn elements(&self) -> &[char] {
    //     &self.data
    // }

    pub fn get(&self, row: isize, column: isize) -> Option<char> {
        let index = self.index_of(row, column)?;
        Some(self.data[index])
    }

    // pub fn set(&mut self, row: usize, column: usize, value: char) {
    //     let index = self.index_of(row, column);
    //     if let Some(index) = index {
    //         self.data[index] = value;
    //     }
    // }

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

        assert_eq!(output, "18")
    }
}

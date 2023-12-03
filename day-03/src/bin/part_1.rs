fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct PartNumber {
    number: String,
    adjency_symbols: u32,
}

fn check_neighbours(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let mut adjency_symbols = 0;

    let rows = matrix.len();
    let cols = matrix[0].len();

    for x in 0..3 {
        for y in 0..3 {
            let ni = row as i32 + x - 1;
            let nj = col as i32 + y - 1;

            if x == 1 && y == 1 {
                continue;
            }

            if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                let c = matrix[ni as usize][nj as usize];
                if c != '.' && !c.is_ascii_digit() {
                    adjency_symbols += 1;
                }
            }
        }
    }

    adjency_symbols
}

fn part_1(input: &str) -> u32 {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    // Convert input to a matrix of chars
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        matrix.push(chars);
    }

    let mut parts = Vec::<PartNumber>::new();
    let mut current_number = PartNumber {
        number: String::new(),
        adjency_symbols: 0,
    };

    for (row, line) in matrix.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if c.is_numeric() {
                current_number.number.push(*c);
                current_number.adjency_symbols += check_neighbours(&matrix, row, col);
            } else {
                if current_number.adjency_symbols > 0 {
                    parts.push(current_number.clone());
                }
                current_number.number = String::new();
                current_number.adjency_symbols = 0;
            }

            if col == line.len() - 1 {
                if current_number.adjency_symbols > 0 {
                    parts.push(current_number.clone());
                }
                current_number.number = String::new();
                current_number.adjency_symbols = 0;
            }
        }
    }

    parts
        .iter()
        .map(|p| p.number.parse::<u32>().expect("Expected an integer"))
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_output() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = part_1(input);
        assert_eq!(result, 4361)
    }
}

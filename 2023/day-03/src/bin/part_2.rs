use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct PartNumber {
    number: String,
    attached_gears: HashSet<(usize, usize)>,
}

fn attached_gears(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut gears = Vec::<(usize, usize)>::new();

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
                if c == '*' {
                    gears.push((ni as usize, nj as usize));
                }
            }
        }
    }

    gears
}

fn part_2(input: &str) -> u32 {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    // Convert input to a matrix of chars
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        matrix.push(chars);
    }

    let mut parts = Vec::<PartNumber>::new();
    let mut current_number = PartNumber {
        number: String::new(),
        attached_gears: HashSet::new(),
    };

    let mut gears_map = HashMap::<(usize, usize), Vec<u32>>::new();

    for (row, line) in matrix.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if c.is_numeric() {
                current_number.number.push(*c);
                let gears = attached_gears(&matrix, row, col);
                current_number.attached_gears.extend(gears);
            } else {
                if current_number.attached_gears.len() > 0 {
                    parts.push(current_number.clone());
                    for gear in current_number.attached_gears.iter() {
                        let gear_numbers = gears_map.entry(*gear).or_insert(Vec::<u32>::new());
                        gear_numbers.push(
                            current_number
                                .number
                                .parse::<u32>()
                                .expect("Expected an integer"),
                        );
                    }
                }
                current_number.number = String::new();
                current_number.attached_gears = HashSet::new();
            }

            if col == line.len() - 1 {
                if current_number.attached_gears.len() > 0 {
                    parts.push(current_number.clone());
                }
                current_number.number = String::new();
                current_number.attached_gears = HashSet::new();
            }
        }
    }

    gears_map
        .iter()
        .map(|(_, v)| {
            if v.len() == 1 {
                return 0;
            }
            v.iter().product()
        })
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
        let result = part_2(input);
        assert_eq!(result, 467835)
    }
}

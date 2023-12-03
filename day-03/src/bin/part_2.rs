fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> u32 {
    // let possible_ids: u32 = input.lines();
    todo!();
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
        assert_eq!(result, 8)
    }
}

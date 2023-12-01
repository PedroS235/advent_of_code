fn main() {
    let input = include_str!("./input1.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> i32 {
    let output = input.lines().map(process_line).sum();
    output
}

fn process_line(string: &str) -> i32 {
    let mut number = String::new();
    number.push(find_first_value(string));
    number.push(find_last_value(string));
    number.parse::<i32>().unwrap_or_default()
}

fn find_first_value(string: &str) -> char {
    for char in string.chars() {
        if char.is_numeric() {
            return char;
        }
    }
    return ' ';
}

fn find_last_value(string: &str) -> char {
    for char in string.chars().rev() {
        if char.is_numeric() {
            return char;
        }
    }
    return ' ';
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_output() {
        let result = part_1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
        assert_eq!(result, 142)
    }
}

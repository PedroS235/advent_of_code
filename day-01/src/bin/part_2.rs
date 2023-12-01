fn main() {
    let input = include_str!("./input1.txt");
    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> i32 {
    let output = input.lines().map(process_line).sum();
    output
}

fn process_line(string: &str) -> i32 {
    let mut number = String::new();

    let mut processed_string = string.replace("one", "o1ne");
    processed_string = processed_string.replace("two", "t2wo");
    processed_string = processed_string.replace("three", "th3ree");
    processed_string = processed_string.replace("four", "fo4ur");
    processed_string = processed_string.replace("five", "fiv5e");
    processed_string = processed_string.replace("six", "si6x");
    processed_string = processed_string.replace("seven", "seve7n");
    processed_string = processed_string.replace("eight", "ei8ght");
    processed_string = processed_string.replace("nine", "ni9ne");
    println!("{processed_string}");

    number.push(find_first_value(processed_string.as_str()));
    number.push(find_last_value(processed_string.as_str()));
    println!("{number}");
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
        let result = part_2(
            "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen",
        );
        assert_eq!(result, 281)
    }
}

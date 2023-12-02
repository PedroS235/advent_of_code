fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> u32 {
    let possible_ids: u32 = input
        .lines()
        .map(|line| {
            let mut game = line.split(": ");
            let game_id = get_game_id(game.next().expect("game id"));
            if valid_bag(game.last().expect("bag of items")) {
                game_id
            } else {
                0
            }
        })
        .sum();

    possible_ids
}

fn get_game_id(string: &str) -> u32 {
    string
        .split(' ')
        .last()
        .expect("Expected something of the form Game id")
        .parse::<u32>()
        .expect("Expected an Integer")
}

fn valid_bag(bag: &str) -> bool {
    let valid: u32 = bag
        .split("; ")
        .flat_map(|config| {
            config.split(", ").filter_map(|color| {
                let (color_name, value_str) = if color.contains("red") {
                    ("red", color.replace(" red", ""))
                } else if color.contains("blue") {
                    ("blue", color.replace(" blue", ""))
                } else if color.contains("green") {
                    ("green", color.replace(" green", ""))
                } else {
                    return None;
                };

                if let Ok(value) = value_str.parse::<u32>() {
                    if (color_name == "red" && value <= 12)
                        || (color_name == "blue" && value <= 14)
                        || (color_name == "green" && value <= 13)
                    {
                        Some(0)
                    } else {
                        Some(value)
                    }
                } else {
                    None
                }
            })
        })
        .sum();

    valid == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_output() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let result = part_1(input);
        assert_eq!(result, 8)
    }
}

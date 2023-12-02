fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

#[derive(Debug)]
struct CubeColors {
    red: u32,
    blue: u32,
    green: u32,
}

fn part_1(input: &str) -> u32 {
    let possible_ids: u32 = input
        .lines()
        .map(|line| {
            let game = line.split(": ");
            let val = power_set(game.last().expect("bag of items"));
            val
        })
        .sum();

    possible_ids
}

fn power_set(bag: &str) -> u32 {
    let mut min_set = CubeColors {
        red: 0,
        blue: 0,
        green: 0,
    };

    bag.split("; ").for_each(|config| {
        config.split(", ").for_each(|color| {
            if color.contains("red") {
                let value = color
                    .replace(" red", "")
                    .parse::<u32>()
                    .expect("Expected a value");

                if value > min_set.red {
                    min_set.red = value;
                }
            }
            if color.contains("blue") {
                let value = color
                    .replace(" blue", "")
                    .parse::<u32>()
                    .expect("Expected a value");

                if value > min_set.blue {
                    min_set.blue = value;
                }
            }
            if color.contains("green") {
                let value = color
                    .replace(" green", "")
                    .parse::<u32>()
                    .expect("Expected a value");

                if value > min_set.green {
                    min_set.green = value;
                }
            }
        })
    });

    min_set.red * min_set.blue * min_set.green
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
        assert_eq!(result, 2286)
    }
}

use std::iter::zip;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

struct Race {
    time: u32,
    distance: u32,
}

fn parse_races(input: &str) -> (u64, u64) {
    let times = input
        .lines()
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .chars()
        .filter(|x| !x.is_whitespace())
        .collect::<Vec<char>>();
    let distances = input
        .lines()
        .nth(1)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .chars()
        .filter(|x| !x.is_whitespace())
        .collect::<Vec<char>>();

    let times = String::from_iter(times).parse().unwrap();
    let distances = String::from_iter(distances).parse().unwrap();
    (times, distances)
}

fn part_1(input: &str) -> u64 {
    let (times, distances) = parse_races(input);
    let mut ways = 1;

    let mut wins = 0;
    for time in 1..(times) {
        let total_dist = time * (times - time);
        if total_dist > distances {
            wins += 1;
        }
    }
    ways *= wins;

    ways
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_output() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = part_1(input);
        assert_eq!(result, 71503)
    }
}

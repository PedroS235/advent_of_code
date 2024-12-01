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

fn parse_races(input: &str) -> (Vec<u32>, Vec<u32>) {
    let times = input
        .lines()
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    let distances = input
        .lines()
        .nth(1)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    (times, distances)
}

fn part_1(input: &str) -> u64 {
    let (times, distances) = parse_races(input);

    let mut ways = 1;

    for (race_time, race_dist) in zip(times, distances) {
        let mut wins = 0;
        for time in 1..(race_time) {
            let total_dist = time * (race_time - time);
            if total_dist > race_dist {
                wins += 1;
            }
        }
        ways *= wins;
    }

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
        assert_eq!(result, 288)
    }
}

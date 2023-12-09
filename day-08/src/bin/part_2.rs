use num::integer::lcm;
use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input);
    dbg!(output);
}

fn parse_maps(input: Vec<&str>) -> BTreeMap<String, (String, String)> {
    input
        .iter()
        .map(|line| {
            let (key, value) = line.split_once(" = ").unwrap();
            let value = value.replace("(", "");
            let value = value.replace(")", "");
            let (left, right) = value.split_once(", ").unwrap();
            (key.to_string(), left.to_string(), right.to_string())
        })
        .fold(BTreeMap::new(), |mut acc, m| {
            let key = m.0;
            let paths = (m.1, m.2);
            acc.insert(key, paths);
            acc
        })
}

fn part_2(input: &str) -> usize {
    let mut network = input.lines();
    let instructions: Vec<char> = network.next().unwrap().chars().collect();

    // Skipping the empty line
    network.next();

    let maps = parse_maps(network.collect());

    let start_points: Vec<&String> = maps.keys().filter(|k| k.ends_with('A')).collect();
    let mut total_steps = 1;

    for start_point in start_points {
        let mut reach_destination = false;
        let mut next_dest = start_point.clone();
        let mut steps = 0;
        let mut curr_instruction = 0;

        while !reach_destination {
            next_dest = match instructions[curr_instruction] {
                'L' => maps.get(&next_dest).unwrap().0.clone(),
                'R' => maps.get(&next_dest).unwrap().1.clone(),
                value => panic!("Value {} should not exist!", value),
            };

            steps += 1;

            if next_dest.ends_with("Z") {
                reach_destination = true;
            }
            curr_instruction = (curr_instruction + 1) % instructions.len();
        }

        total_steps = lcm(steps, total_steps);
    }

    total_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = part_2(input);
        assert_eq!(result, 6)
    }
}

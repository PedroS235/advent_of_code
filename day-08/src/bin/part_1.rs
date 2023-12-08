use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
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

fn part_1(input: &str) -> u32 {
    let mut network = input.lines();
    let instructions: Vec<char> = network.next().unwrap().chars().collect();

    // Skipping the empty line
    network.next();

    let maps = parse_maps(network.collect());

    let mut curr_instruction = 0;

    let mut reach_destination = false;

    let mut next_dest = "AAA";

    let mut steps = 0;

    while !reach_destination {
        next_dest = match instructions[curr_instruction] {
            'L' => maps.get(next_dest).unwrap().0.as_str(),
            'R' => maps.get(next_dest).unwrap().1.as_str(),
            value => panic!("Value {} should not exist!", value),
        };

        steps += 1;

        if next_dest == "ZZZ" {
            reach_destination = true;
        }
        curr_instruction = (curr_instruction + 1) % instructions.len();
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result = part_1(input);
        assert_eq!(result, 2)
    }

    #[test]
    fn test_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = part_1(input);
        assert_eq!(result, 6)
    }
}

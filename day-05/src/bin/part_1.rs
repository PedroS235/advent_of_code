use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil: HashMap<u32, u32>,
    soil_to_fert: HashMap<u32, u32>,
    fert_to_water: HashMap<u32, u32>,
    water_to_light: HashMap<u32, u32>,
    light_to_temp: HashMap<u32, u32>,
    temp_to_hum: HashMap<u32, u32>,
    hum_to_loc: HashMap<u32, u32>,
}

struct MapDescription {
    source_start: u32,
    target_start: u32,
    offset: u32,
}

fn parse_seeds(line: &str) -> Vec<u32> {
    line.split(' ').map(|n| n.parse::<u32>().unwrap()).collect()
}

fn parse_map_description(line: &str) -> MapDescription {
    let mut desc = line
        .split(' ')
        .map(|n| n.parse::<u32>().unwrap())
        .into_iter();
    MapDescription {
        source_start: (desc.nth(0).unwrap()),
        target_start: (desc.nth(1).unwrap()),
        offset: (desc.nth(2).unwrap()),
    }
}

fn part_1(input: &str) -> u32 {
    // Convert input to a matrix of chars
    for line in input.lines() {
        println!("{}", line);
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_output() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = part_1(input);
        assert_eq!(result, 4361)
    }
}

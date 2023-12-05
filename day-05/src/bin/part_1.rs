use std::collections::BTreeMap;
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

impl MapDescription {
    fn gen_ranges(&self) -> (Vec<u32>, Vec<u32>) {
        let source: Vec<u32> = (self.source_start..self.offset).collect();
        let target: Vec<u32> = (self.target_start..self.offset).collect();
        (source, target)
    }

    fn new() -> Self {
        MapDescription {
            source_start: 0,
            target_start: 0,
            offset: 0,
        }
    }
}

fn parse_seeds(line: &str) -> Vec<u32> {
    line.split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
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

fn parse_map(maps: Vec<&str>) -> HashMap<u32, u32> {
    let mut hash_map: HashMap<u32, u32> = HashMap::new();
    for line in maps {
        let map_desc = parse_map_description(line);
        let (source, target) = map_desc.gen_ranges();
        let mut target = target.into_iter();

        for (i, key) in source.into_iter().enumerate() {
            hash_map.insert(key, target.nth(i).unwrap());
        }
    }
    hash_map
}

fn part_1(input: &str) -> u32 {
    let mut lines = input.lines();
    let seeds = parse_seeds(lines.nth(0).unwrap());

    let mut current_map: u32 = 0;
    let mut maps: Vec<&str> = Vec::new();

    for (i, line) in lines.enumerate() {
        if i < 1 {
            continue;
        }
        println!("{line}");

        // Beginning of map
        if line.contains("map") {
            maps = Vec::new();
            continue;
        }

        // End of map
        if line == "" {
            current_map += 1;

            match current_map {
                0 => {
                    let map = parse_map(maps.clone());
                }

                1 => {
                    let map = parse_map(maps.clone());
                }
                2 => {
                    let map = parse_map(maps.clone());
                }
                3 => {
                    let map = parse_map(maps.clone());
                }
                4 => {
                    let map = parse_map(maps.clone());
                }
                5 => {
                    let map = parse_map(maps.clone());
                }
                6 => {
                    let map = parse_map(maps.clone());
                }
                7 => {
                    let map = parse_map(maps.clone());
                }

                _ => {
                    println!("Other")
                }
            }

            continue;
        }
        maps.push(line);
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

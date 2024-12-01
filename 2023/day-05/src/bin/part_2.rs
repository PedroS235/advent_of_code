use std::{cmp::min, collections::HashSet};
fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Seed {
    start_range: u64,
    offset: u64,
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<Seed>,
    seed_to_soil: Vec<MapDescription>,
    soil_to_fert: Vec<MapDescription>,
    fert_to_water: Vec<MapDescription>,
    water_to_light: Vec<MapDescription>,
    light_to_temp: Vec<MapDescription>,
    temp_to_hum: Vec<MapDescription>,
    hum_to_loc: Vec<MapDescription>,
}

impl Almanac {
    fn new() -> Self {
        Almanac {
            seeds: Vec::new(),
            seed_to_soil: Vec::new(),
            soil_to_fert: Vec::new(),
            fert_to_water: Vec::new(),
            water_to_light: Vec::new(),
            light_to_temp: Vec::new(),
            temp_to_hum: Vec::new(),
            hum_to_loc: Vec::new(),
        }
    }

    fn create(&mut self, input: &str) {
        let mut lines = input.lines();
        self.seeds = parse_seeds(lines.nth(0).unwrap());

        let mut current_map: u64 = 0;
        let mut maps: Vec<&str> = Vec::new();

        for (i, line) in lines.enumerate() {
            if i < 1 {
                continue;
            }

            // Beginning of map
            if line.contains("map") {
                maps = Vec::new();
                continue;
            }

            // End of map
            if line == "" {
                match current_map {
                    0 => self.seed_to_soil = parse_map(maps.clone()),
                    1 => self.soil_to_fert = parse_map(maps.clone()),
                    2 => self.fert_to_water = parse_map(maps.clone()),
                    3 => self.water_to_light = parse_map(maps.clone()),
                    4 => self.light_to_temp = parse_map(maps.clone()),
                    5 => self.temp_to_hum = parse_map(maps.clone()),
                    6 => self.hum_to_loc = parse_map(maps.clone()),
                    _ => println!("Something went wrong! Only expecting 7 maps."),
                }
                current_map += 1;

                continue;
            }
            maps.push(line);
        }
    }

    fn soil_to_location(&self, seed: u64) -> u64 {
        let mut location = seed;

        for map in self.seed_to_soil.clone() {
            if location >= map.source_start && location < map.source_start + map.offset {
                location = map.target_start + location - map.source_start;
                break;
            }
        }
        for map in self.soil_to_fert.clone() {
            if location >= map.source_start && location < map.source_start + map.offset {
                location = map.target_start + location - map.source_start;
                break;
            }
        }
        for map in self.fert_to_water.clone() {
            if location >= map.source_start && location < map.source_start + map.offset {
                location = map.target_start + location - map.source_start;
                break;
            }
        }
        for map in self.water_to_light.clone() {
            if location >= map.source_start && location < map.source_start + map.offset {
                location = map.target_start + location - map.source_start;
                break;
            }
        }
        for map in self.light_to_temp.clone() {
            if location >= map.source_start && location < map.source_start + map.offset {
                location = map.target_start + location - map.source_start;
                break;
            }
        }
        for map in self.temp_to_hum.clone() {
            if location >= map.source_start && location < map.source_start + map.offset {
                location = map.target_start + location - map.source_start;
                break;
            }
        }

        for map in self.hum_to_loc.clone() {
            if location >= map.source_start && location < map.source_start + map.offset {
                location = map.target_start + location - map.source_start;
                break;
            }
        }

        location
    }
}

#[derive(Debug, Clone)]

struct MapDescription {
    source_start: u64,
    target_start: u64,
    offset: u64,
}

fn parse_seeds(line: &str) -> Vec<Seed> {
    let mut seeds: Vec<Seed> = Vec::new();
    let seeds_raw: Vec<u64> = line
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    for seed in (0..seeds_raw.len()).step_by(2) {
        let start_range = seeds_raw[seed];
        let offset = seeds_raw[seed + 1];
        let seed = Seed {
            start_range,
            offset,
        };
        seeds.push(seed);
    }

    seeds
}

fn parse_map_description(line: &str) -> MapDescription {
    let desc = line
        .split(' ')
        .map(|n| n.parse::<u64>().expect("A number"))
        .collect::<Vec<u64>>();

    MapDescription {
        source_start: (*desc.get(1).unwrap()),
        target_start: (*desc.get(0).unwrap()),
        offset: (*desc.get(2).unwrap()),
    }
}

fn parse_map(maps: Vec<&str>) -> Vec<MapDescription> {
    maps.iter()
        .map(|line| parse_map_description(line))
        .collect()
}

fn part_2(input: &str) -> u64 {
    let mut almanac = Almanac::new();
    almanac.create(input);

    let mut min_: u64 = 999999999999999;

    for seed in almanac.seeds.clone() {
        println!("Processing one range");

        let range = seed.start_range..seed.start_range + seed.offset - 1;

        for i in range {
            min_ = min(almanac.soil_to_location(i), min_);
        }
    }

    // for seed in seeds {}

    min_
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
56 93 4

";
        let result = part_2(input);
        assert_eq!(result, 46)
    }
}

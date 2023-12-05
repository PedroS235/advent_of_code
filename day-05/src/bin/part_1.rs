use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil: BTreeMap<u32, u32>,
    soil_to_fert: BTreeMap<u32, u32>,
    fert_to_water: BTreeMap<u32, u32>,
    water_to_light: BTreeMap<u32, u32>,
    light_to_temp: BTreeMap<u32, u32>,
    temp_to_hum: BTreeMap<u32, u32>,
    hum_to_loc: BTreeMap<u32, u32>,
}

impl Almanac {
    fn new() -> Self {
        Almanac {
            seeds: Vec::new(),
            seed_to_soil: BTreeMap::new(),
            soil_to_fert: BTreeMap::new(),
            fert_to_water: BTreeMap::new(),
            water_to_light: BTreeMap::new(),
            light_to_temp: BTreeMap::new(),
            temp_to_hum: BTreeMap::new(),
            hum_to_loc: BTreeMap::new(),
        }
    }

    fn create(&mut self, input: &str) {
        let mut lines = input.lines();
        self.seeds = parse_seeds(lines.nth(0).unwrap());

        let mut current_map: u32 = 0;
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
}

#[derive(Debug, Clone)]

struct MapDescription {
    source_start: u32,
    target_start: u32,
    offset: u32,
}

impl MapDescription {
    fn gen_ranges(&self) -> (Vec<u32>, Vec<u32>) {
        let source: Vec<u32> = (self.source_start..(self.source_start + self.offset)).collect();
        let target: Vec<u32> = (self.target_start..(self.target_start + self.offset)).collect();
        (source, target)
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
    let desc = line
        .split(' ')
        .map(|n| n.parse::<u32>().expect("A number"))
        .collect::<Vec<u32>>();

    MapDescription {
        source_start: (*desc.get(1).unwrap()),
        target_start: (*desc.get(0).unwrap()),
        offset: (*desc.get(2).unwrap()),
    }
}

fn parse_map(maps: Vec<&str>) -> BTreeMap<u32, u32> {
    let mut hash_map: BTreeMap<u32, u32> = BTreeMap::new();
    for line in maps {
        let map_desc = parse_map_description(line);
        let (source, target) = map_desc.gen_ranges();

        for (i, key) in source.into_iter().enumerate() {
            hash_map.insert(key, target[i]);
        }
    }
    hash_map
}

fn part_1(input: &str) -> u32 {
    let mut almanac = Almanac::new();
    almanac.create(input);
    println!("Finished creating almanac");

    let mut locations: Vec<u32> = Vec::new();

    for seed in almanac.seeds {
        let soil = *almanac.seed_to_soil.get(&seed).unwrap_or(&seed);
        let fertilizer = *almanac.soil_to_fert.get(&soil).unwrap_or(&soil);
        let water = *almanac
            .fert_to_water
            .get(&fertilizer)
            .unwrap_or(&fertilizer);
        let light = *almanac.water_to_light.get(&water).unwrap_or(&water);
        let temp = *almanac.light_to_temp.get(&light).unwrap_or(&light);
        let humidity = *almanac.temp_to_hum.get(&temp).unwrap_or(&temp);
        let location = *almanac.hum_to_loc.get(&humidity).unwrap_or(&humidity);

        locations.push(location);
    }

    let min = locations.iter().min().unwrap();

    *min
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
        let result = part_1(input);
        assert_eq!(result, 35)
    }
}

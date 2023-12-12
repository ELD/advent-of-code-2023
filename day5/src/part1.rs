use std::{collections::HashMap, ops::Range, fs::read_to_string};

type Seeds = Vec<usize>;
type SeedSoilMap = HashMap<Range<usize>, Range<usize>>;
type SoilFertilizerMap = HashMap<Range<usize>, Range<usize>>;
type FertilizerWaterMap = HashMap<Range<usize>, Range<usize>>;
type WaterLightMap = HashMap<Range<usize>, Range<usize>>;
type LightTemperatureMap = HashMap<Range<usize>, Range<usize>>;
type TemperatureHumidityMap = HashMap<Range<usize>, Range<usize>>;
type HumidityLocationMap = HashMap<Range<usize>, Range<usize>>;

pub fn solution() -> usize {
    let input = read_to_string("day5/input.txt").expect("unable to read file");
    let (seeds, almanac) = parse_input(&input);
    almanac.lowest_location(&seeds)
}

#[derive(Debug)]
struct Almanac {
    seed_soil_map: SeedSoilMap,
    soil_fertilizer_map: SoilFertilizerMap,
    fertilizer_water_map: FertilizerWaterMap,
    water_light_map: WaterLightMap,
    light_temperature_map: LightTemperatureMap,
    temperature_humidity_map: TemperatureHumidityMap,
    humidity_location_map: HumidityLocationMap,
}

impl Almanac {
    fn lowest_location(&self, seeds: &Seeds) -> usize {
        seeds
            .iter()
            .map(|seed| {
                let soil_loc = self
                    .seed_soil_map
                    .iter()
                    .find_map(|(src, dest)| {
                        if src.contains(seed) {
                            let offset = seed - src.start;
                            return Some(dest.start + offset);
                        }
                        None
                    })
                    .unwrap_or(*seed);
                let fertilizer_loc = self
                    .soil_fertilizer_map
                    .iter()
                    .find_map(|(src, dest)| {
                        if src.contains(&soil_loc) {
                            let offset = soil_loc - src.start;
                            return Some(dest.start + offset);
                        }
                        None
                    })
                    .unwrap_or(soil_loc);
                let water_loc = self
                    .fertilizer_water_map
                    .iter()
                    .find_map(|(src, dest)| {
                        if src.contains(&fertilizer_loc) {
                            let offset = fertilizer_loc - src.start;
                            return Some(dest.start + offset);
                        }
                        None
                    })
                    .unwrap_or(fertilizer_loc);
                let light_loc = self
                    .water_light_map
                    .iter()
                    .find_map(|(src, dest)| {
                        if src.contains(&water_loc) {
                            let offset = water_loc - src.start;
                            return Some(dest.start + offset);
                        }
                        None
                    })
                    .unwrap_or(water_loc);
                let temperature_loc = self
                    .light_temperature_map
                    .iter()
                    .find_map(|(src, dest)| {
                        if src.contains(&light_loc) {
                            let offset = light_loc - src.start;
                            return Some(dest.start + offset);
                        }
                        None
                    })
                    .unwrap_or(light_loc);
                let humidity_loc = self
                    .temperature_humidity_map
                    .iter()
                    .find_map(|(src, dest)| {
                        if src.contains(&temperature_loc) {
                            let offset = temperature_loc - src.start;
                            return Some(dest.start + offset);
                        }
                        None
                    })
                    .unwrap_or(temperature_loc);
                self.humidity_location_map
                    .iter()
                    .find_map(|(src, dest)| {
                        if src.contains(&humidity_loc) {
                            let offset = humidity_loc - src.start;
                            return Some(dest.start + offset)
                        }
                        None
                    }).unwrap_or(humidity_loc)
            })
            .min()
            .unwrap_or(0)
    }
}

fn parse_input(input: &str) -> (Seeds, Almanac) {
    let mut parts = input.split("\n\n");

    let seeds = parts.next().expect("malformed input; no seeds");
    let seed_soil = parts.next().expect("malformed input; no seed-to-soil maps");
    let soil_fertilizer = parts
        .next()
        .expect("malformed input; no soil-to-fertilizer map");
    let fertilizer_water = parts
        .next()
        .expect("malformed input; fertilizer-to-water map");
    let water_light = parts
        .next()
        .expect("malformed input; no water-to-light map");
    let light_temperature = parts
        .next()
        .expect("malformed input; no light-to-temperature map");
    let temperature_humidity = parts
        .next()
        .expect("malformed input; no temperature-to-humidity map");
    let humidity_location = parts
        .next()
        .expect("malformed input; no humidity-to-location map");

    let seeds = parse_seeds(seeds);

    (
        seeds,
        Almanac {
            seed_soil_map: parse_seed_soil_map(seed_soil),
            soil_fertilizer_map: parse_soil_fertilizer_map(soil_fertilizer),
            fertilizer_water_map: parse_fertilizer_water_map(fertilizer_water),
            water_light_map: parse_water_light_map(water_light),
            light_temperature_map: parse_light_temperature_map(light_temperature),
            temperature_humidity_map: parse_temperature_humidity_map(temperature_humidity),
            humidity_location_map: parse_humidity_location_map(humidity_location),
        },
    )
}

fn parse_seeds(input: &str) -> Seeds {
    input
        .split(':')
        .nth(1)
        .expect("malformed seeds")
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("invalid number"))
        .collect()
}

macro_rules! parse_map {
    ($name:ident, $type:ty) => {
        fn $name(input: &str) -> $type {
            let mut map = <$type>::new();
            input
                .split(":\n")
                .nth(1)
                .expect("malformed map")
                .lines()
                .for_each(|line| {
                    let mut parts = line.split(" ");
                    let destination = parts
                        .next()
                        .expect("malformed map")
                        .parse::<usize>()
                        .expect("invalid number");
                    let source = parts
                        .next()
                        .expect("malformed map")
                        .parse::<usize>()
                        .expect("invalid number");
                    let range = parts
                        .next()
                        .expect("malformed map")
                        .parse::<usize>()
                        .expect("invalid number");

                    map.insert(source..source + range, destination..destination + range);
                });
            map
        }
    };
}

parse_map!(parse_seed_soil_map, SeedSoilMap);
parse_map!(parse_soil_fertilizer_map, SoilFertilizerMap);
parse_map!(parse_fertilizer_water_map, FertilizerWaterMap);
parse_map!(parse_water_light_map, WaterLightMap);
parse_map!(parse_light_temperature_map, LightTemperatureMap);
parse_map!(parse_temperature_humidity_map, TemperatureHumidityMap);
parse_map!(parse_humidity_location_map, HumidityLocationMap);

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"seeds: 79 14 55 13

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

    #[test]
    fn example() {
        let (seeds, almanac) = parse_input(INPUT);
        assert_eq!(almanac.lowest_location(&seeds), 35);
    }
}

struct Range {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn needs_converted(&self, seed: u64) -> bool {
        for range in self.ranges.iter() {
            if (range.source_start..(range.source_start + range.length)).contains(&seed) {
                return true;
            }
        }
        false
    }

    fn convert(&self, seed: u64) -> u64 {
        for range in self.ranges.iter() {
            if (range.source_start..(range.source_start + range.length)).contains(&seed) {
                let diff = seed - range.source_start;
                let new_seed = range.destination_start + diff;
                return new_seed;
            }
        }
        seed
    }
}

fn parse_input(input: String) -> (Vec<u64>, Vec<Map>) {
    let mut seeds: Vec<u64> = vec![];
    let mut almanac_map: Vec<Map> = vec![];

    let _ = input
        .split("\n\n")
        .map(|x| {
            let mut split_maps = x.split(':');
            let map_name = split_maps.next().unwrap();
            let values = split_maps.next().unwrap();
            let parsed_values = values
                .split('\n')
                .filter(|x| !x.is_empty())
                .map(|x| {
                    x.split_ascii_whitespace()
                        .filter_map(|x| x.parse::<u64>().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            if map_name == "seeds" {
                let value = &parsed_values[0];
                seeds = value.clone();
            } else {
                let mut ranges: Vec<Range> = vec![];
                for value in parsed_values {
                    ranges.push(Range {
                        destination_start: value[0],
                        source_start: value[1],
                        length: value[2],
                    });
                }
                almanac_map.push(Map { ranges })
            }
        })
        .collect::<Vec<_>>();

    (seeds, almanac_map)
}

fn part_one(seeds: &[u64], almanac_map: &[Map]) -> u64 {
    let mut locations: Vec<u64> = vec![];

    for seed in seeds {
        let mut seed_value: u64 = *seed;

        for map in almanac_map.iter() {
            if map.needs_converted(seed_value) {
                seed_value = map.convert(seed_value);
            }
        }
        locations.push(seed_value);
    }
    *locations.iter().min().unwrap()
}

fn get_min_max_seeds(seeds: &[u64]) -> Vec<(u64, u64)> {
    let mut min_max_seeds: Vec<(u64, u64)> = vec![];
    let chunks = seeds.chunks(2);

    for chunk in chunks {
        let seed_start = chunk[0];
        let seed_end = seed_start + chunk[1];
        min_max_seeds.push((seed_start, seed_end));
    }
    min_max_seeds
}

fn part_two(seeds: &[u64], almanac_map: &[Map]) -> u64 {
    let mut locations: Vec<u64> = vec![];
    let chunks = get_min_max_seeds(seeds);

    for (low, high) in chunks {
        let mut values = (low..high).collect::<Vec<_>>();
        let mut cur_low = low;
        let mut cur_high = high;

        // to try and make this brute force solution less brutal...
        // only attempt to convert the values if the lowest or highest value needs converted.
        // this could probably be optimised futher to exclude more values.
        for map in almanac_map.iter() {
            if map.needs_converted(cur_low) || map.needs_converted(cur_high) {
                for (_, v) in values.iter_mut().enumerate() {
                    let new_value = map.convert(*v);
                    *v = new_value;
                }
                cur_low = *values.iter().min().unwrap();
                cur_high = *values.iter().max().unwrap();
            }
        }
        locations.push(cur_low);
    }
    *locations.iter().min().unwrap()
}

fn main() {
    let input = std::fs::read_to_string("day5.txt").unwrap();
    let (seeds, almanac_map) = parse_input(input);

    let part_one = part_one(&seeds, &almanac_map);
    let part_two = part_two(&seeds, &almanac_map);
    println!("part 1: {}, part 2: {}", part_one, part_two);
}

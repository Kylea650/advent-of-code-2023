use std::collections::HashMap;

fn parse_input_part_one(input: &str) -> HashMap<&str, Vec<u32>> {
    input
        .lines()
        .map(|x| {
            let mut split = x.split(':');
            let label = split.next().unwrap();
            let values = split.next().unwrap();
            let converted_values: Vec<u32> = values
                .split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            (label, converted_values)
        })
        .collect()
}

fn parse_input_part_two(input: &str) -> HashMap<&str, u64> {
    input
        .lines()
        .map(|x| {
            let mut split = x.split(':');
            let label = split.next().unwrap();
            let values = split.next().unwrap();
            let converted_values = values
                .chars()
                .filter(|x| !x.is_whitespace())
                .collect::<String>();

            (label, converted_values.parse::<u64>().unwrap())
        })
        .collect()
}

fn part_one(races: HashMap<&str, Vec<u32>>) -> u32 {
    let times = races.get("Time").unwrap();
    let distances = races.get("Distance").unwrap();

    let mut records_broke: Vec<u32> = vec![];

    for (i, time) in times.iter().enumerate() {
        let record = *distances.get(i).unwrap() as i32;
        let mut counter: u32 = 0;
        let mut index: i32 = (((time + 2) / 2) - 1) as i32;
        let mut distance: i32 =
            -(index + 1).pow(2) + (*time as i32 + 2) * (index + 1) - (*time as i32 + 1);

        while distance > record {
            counter += 1;
            index -= 1;
            distance = -(index + 1).pow(2) + (*time as i32 + 2) * (index + 1) - (*time as i32 + 1);
        }
        match time % 2 {
            1 => counter *= 2,
            0 => counter = (counter - 1) * 2 + 1,
            _ => panic!(),
        }
        records_broke.push(counter)
    }
    records_broke.into_iter().product()
}

fn part_two(races: HashMap<&str, u64>) -> u32 {
    let time = races.get("Time").unwrap();
    let distance = races.get("Distance").unwrap();

    let record = *distance as i64;
    let mut counter: u32 = 0;
    let mut index: i64 = (((time + 2) / 2) - 1) as i64;
    let mut distance: i64 =
        -(index + 1).pow(2) + (*time as i64 + 2) * (index + 1) - (*time as i64 + 1);

    while distance > record {
        counter += 1;
        index -= 1;
        distance = -(index + 1).pow(2) + (*time as i64 + 2) * (index + 1) - (*time as i64 + 1);
    }
    match time % 2 {
        1 => counter *= 2,
        0 => counter = (counter - 1) * 2 + 1,
        _ => panic!(),
    }
    counter
}

fn main() {
    let input = std::fs::read_to_string("day6.txt").unwrap();

    let races_part_one = parse_input_part_one(&input);
    let races_part_two = parse_input_part_two(&input);

    let part_one = part_one(races_part_one);
    let part_two = part_two(races_part_two);

    println!("part 1: {}, part 2: {}", part_one, part_two);
}

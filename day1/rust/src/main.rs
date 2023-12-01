use std::collections::HashMap;

fn part_one(input: String) -> u32 {
    let values = input
        .lines()
        .map(|x| {
            x.split("")
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum: u32 = 0;

    for line in values {
        sum += line[0] * 10;
        sum += line[line.len() - 1]
    }
    sum
}

fn part_two(input: String) -> u32 {
    let values = input
        .lines()
        .map(|x| x.split("").map(|x| x.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut sum: u32 = 0;

    for line in values {
        let mut converted_line: Vec<u32> = vec![];
        let mut i: usize = 0;
        let mut word = "".to_string();

        while i < line.len() - 1 {
            let char = line[i].clone();

            if char.parse::<u32>().is_ok() {
                converted_line.push(char.parse::<u32>().unwrap());
                word = "".to_string();
                i += 1;
            } else {
                let mut j: usize = i + 1;
                word.push_str(&char);

                while j < line.len() {
                    let char2 = line[j].clone();
                    word.push_str(&char2);

                    if let Some(x) = map.get(word.as_str()) {
                        converted_line.push(x.parse::<u32>().unwrap());
                        i += 1;
                        break;
                    }
                    j += 1
                }
                word = "".to_string();
                i += 1
            }
        }
        let last = line.last().unwrap().clone();
        if last.parse::<u32>().is_ok() {
            converted_line.push(last.parse::<u32>().unwrap());
        }
        sum += converted_line[0] * 10;
        sum += converted_line[converted_line.len() - 1];
    }
    sum
}

fn main() {
    let input = std::fs::read_to_string("day1.txt").unwrap();

    let part_one = part_one(input.clone());
    let part_two = part_two(input);

    println!("part 1: {}, part 2: {}", part_one, part_two)
}

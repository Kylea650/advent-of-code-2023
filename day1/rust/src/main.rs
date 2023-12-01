use std::collections::HashMap;

fn part_one(input: String) -> u32 {
    // parse input string into 2D vector filtering out any non-numerical values and summing first and last
    input
        .lines()
        .map(|x| {
            let mut vals = x.split("").filter_map(|x| x.parse::<u32>().ok());
            let first = vals.next().unwrap();
            let last = vals.last();
            match last {
                Some(last) => first * 10 + last,
                None => first * 10 + first,
            }
        })
        .sum::<u32>()
}

fn part_two(input: String) -> u32 {
    // parse input string into 2D vector of all characters
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
        let mut word = "".to_string();

        let mut i: usize = 0;

        // iterate through each line. If the character is a number, add it to our list.
        // if not, create a second pointer and start trying to build words
        while i < line.len() - 1 {
            let start_char = line[i].clone();

            match start_char.parse::<u32>() {
                Ok(x) => {
                    converted_line.push(x);
                    word = "".to_string();
                    i += 1;
                }
                Err(_) => {
                    let mut j: usize = i + 1;
                    word.push_str(&start_char);

                    while j < line.len() {
                        let end_char = line[j].clone();
                        word.push_str(&end_char);

                        // check if the word exists in the map and converted it to the associated numberical value
                        if let Some(x) = map.get(word.as_str()) {
                            converted_line.push(x.parse::<u32>().unwrap());
                            i += 1;
                        }
                        j += 1
                    }
                    word = "".to_string();
                    i += 1
                }
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

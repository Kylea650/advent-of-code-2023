#[derive(Debug)]
struct PartNumber {
    value: u32,
    start_coordinate: (usize, usize),
    end_coordinate: (usize, usize),
}

// parses the input for numbers and returns a vector of PartNumbers
fn get_part_numbers(input: &str) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = vec![];

    let _ = input
        .lines()
        .enumerate()
        .map(|(line_idx, v)| {
            let len = v.len();
            let chars = v.chars().enumerate();
            let mut str_number = "".to_string();

            for (char_idx, v) in chars {
                // if we reach a non-number or we reach the end of the row
                if !v.is_ascii_digit() || char_idx == len - 1 {
                    // pushing the last digit to the number string if it's a number
                    if v.is_ascii_digit() {
                        str_number.push(v);
                    }
                    // if the number string isn't empty, create a new PartNumber and push
                    // to the part_numbers vector
                    if !str_number.is_empty() {
                        let number = str_number.parse::<u32>().unwrap();
                        let start_y = char_idx - str_number.len();
                        let end_y = char_idx - 1;

                        part_numbers.push(PartNumber {
                            value: (number),
                            start_coordinate: (line_idx, start_y),
                            end_coordinate: (line_idx, end_y),
                        })
                    }
                    str_number = "".to_string()
                } else {
                    str_number.push(v);
                }
            }
        })
        .collect::<Vec<_>>();
    part_numbers
}

// parses the input and return a vector of all the (x, y) coordinates of all symbols, excluding '.'
fn get_symbol_coordinates(input: &str) -> Vec<(usize, usize)> {
    let mut symbol_coordinates: Vec<(usize, usize)> = vec![];

    let _ = input
        .lines()
        .enumerate()
        .map(|(line_idx, v)| {
            let chars = v
                .chars()
                .enumerate()
                .filter(|(_, v)| *v != '.' && v.is_ascii_punctuation());

            for (char_idx, _) in chars {
                symbol_coordinates.push((line_idx, char_idx))
            }
        })
        .collect::<Vec<_>>();

    symbol_coordinates
}

// checks whether a given PartNumber is adjacent to a symbol
fn adjacent_to_symbol(
    start: (usize, usize),
    end: (usize, usize),
    symbols: &Vec<(usize, usize)>,
) -> bool {
    let mut start_x: usize = start.0;
    let mut start_y: usize = start.1;
    let end_x: usize = end.0;
    let end_y: usize = end.1;

    start_x = match start.0 {
        0 => start_x + 1,
        _ => start_x,
    };

    start_y = match start.1 {
        0 => start_y + 1,
        _ => start_y,
    };

    let x_range = (start_x - 1)..(end_x + 2);
    let y_range = (start_y - 1)..(end_y + 2);

    for symbol in symbols {
        if x_range.contains(&symbol.0) && y_range.contains(&symbol.1) {
            return true;
        }
    }
    false
}

// parses the input and return a vector of all the (x, y) coordinates of all * symbols
fn get_gear_coordinates(input: &str) -> Vec<(usize, usize)> {
    let mut gear_coordinates: Vec<(usize, usize)> = vec![];

    let _ = input
        .lines()
        .enumerate()
        .map(|(line_idx, v)| {
            let chars = v.chars().enumerate().filter(|(_, v)| *v == '*');

            for (char_idx, _) in chars {
                gear_coordinates.push((line_idx, char_idx))
            }
        })
        .collect::<Vec<_>>();

    gear_coordinates
}

// given a * coordinate, iterates over all part numbers and checks if they're adjacent.
// if the * coordinate has more than 1 adjacent part number, we return the product of the part numbers
// otherwise return none
fn get_gear_ratio(coordinate: (usize, usize), part_numbers: &Vec<PartNumber>) -> Option<u32> {
    let mut gear_ratios: Vec<u32> = vec![];
    let mut count: u32 = 0;

    for part_number in part_numbers {
        let number = part_number.value;
        let start = part_number.start_coordinate;
        let end = part_number.end_coordinate;
        let symbols = vec![coordinate];

        if adjacent_to_symbol(start, end, &symbols) {
            count += 1;
            gear_ratios.push(number)
        }
    }
    if count > 1 {
        let gear_ratio = gear_ratios.into_iter().product();
        Some(gear_ratio)
    } else {
        None
    }
}

fn main() {
    let input = std::fs::read_to_string("day3.txt").unwrap();

    let symbol_coordinates = get_symbol_coordinates(&input);
    let part_numbers = get_part_numbers(&input);
    let gear_coordinates = get_gear_coordinates(&input);

    let mut part_one: u32 = 0;

    // for every part number, increment our counter if it's adjacent to a symbol
    for part_number in &part_numbers {
        let number = part_number.value;
        let start = part_number.start_coordinate;
        let end = part_number.end_coordinate;

        if adjacent_to_symbol(start, end, &symbol_coordinates) {
            part_one += number;
        }
    }

    let mut part_two: u32 = 0;

    // for every gear (*), increment our counter with the gear ratio if it is
    // adjacent to at least 2 part numbers
    for coordinate in gear_coordinates {
        let gear_ratio = get_gear_ratio(coordinate, &part_numbers);
        if let Some(x) = gear_ratio {
            part_two += x;
        }
    }

    println!("part 1: {}, part 2: {}", part_one, part_two)
}

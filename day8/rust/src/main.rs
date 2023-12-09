use std::collections::HashMap;

fn parse_input(input: String) -> (Vec<char>, HashMap<String, (String, String)>) {
    let split = input.split("\n\n").collect::<Vec<_>>();

    let str_directions = split[0];
    let directions: Vec<char> = str_directions.chars().collect();

    let str_nodes = split[1];
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    let _: Vec<_> = str_nodes
        .split('\n')
        .map(|x| {
            let mut split = x.split(" = ");
            let node = split.next().unwrap().to_string();
            let elements = split.next().unwrap();
            let (l, r) = elements.split_once(", ").unwrap();
            let left = l.replace('(', "");
            let right = r.replace(')', "");
            nodes.insert(node, (left, right))
        })
        .collect();

    (directions, nodes)
}

fn part_one(directions: &[char], nodes: &HashMap<String, (String, String)>) -> u32 {
    let mut node = nodes.get("AAA").unwrap();
    let mut element = "";
    let mut counter: u32 = 0;

    while element != "ZZZ" {
        for direction in directions.iter() {
            element = match direction {
                'L' => &node.0,
                'R' => &node.1,
                _ => panic!(),
            };
            counter += 1;
            node = nodes.get(element).unwrap();
        }
    }
    counter
}

fn part_two(directions: &[char], nodes: &HashMap<String, (String, String)>) -> u64 {
    let mut starting_nodes: Vec<&String> = vec![];
    for node in nodes.keys() {
        let last_char: Vec<char> = node.chars().rev().take(1).collect();
        if last_char[0] == 'A' {
            starting_nodes.push(node);
        }
    }
    let mut counters: Vec<u64> = vec![];

    for node in starting_nodes {
        let mut elements = nodes.get(node).unwrap();
        let mut counter: u64 = 0;
        let mut last_char: Vec<char> = Vec::from(['A']);

        while last_char[0] != 'Z' {
            for direction in directions.iter() {
                let element = match direction {
                    'L' => &elements.0,
                    'R' => &elements.1,
                    _ => panic!(),
                };
                counter += 1;
                elements = nodes.get(element).unwrap();
                last_char = element.chars().rev().take(1).collect();
                if last_char[0] == 'Z' {
                    break;
                }
            }
        }
        counters.push(counter)
    }
    get_all_lowest_common_multiples(&mut counters)
}

fn get_all_lowest_common_multiples(numbers: &mut Vec<u64>) -> u64 {
    if numbers.len() == 1 {
        return numbers[0];
    }
    let (first, second) = (numbers[0], numbers[1]);
    let current_lcm = lowest_common_multiple(first, second);
    numbers.remove(0);
    numbers.remove(0);
    numbers.push(current_lcm);

    get_all_lowest_common_multiples(numbers)
}

fn lowest_common_multiple(first: u64, second: u64) -> u64 {
    first * second / greatest_common_divisor(first, second)
}

fn greatest_common_divisor(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

fn main() {
    let input = std::fs::read_to_string("day8.txt").unwrap();
    let (directions, nodes) = parse_input(input);
    let part_one = part_one(&directions, &nodes);
    let part_two = part_two(&directions, &nodes);

    println!("part 1: {} part 2: {}", part_one, part_two)
}

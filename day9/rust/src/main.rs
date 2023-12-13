fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|x| x.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()).collect()
}

fn get_diffs(histories: Vec<i32>) -> Vec<i32>{
    let mut diffs: Vec<i32> = vec![];

    for (i, history) in histories.iter().enumerate() {
        if i == histories.len() - 1 {
            break
        }
        let next = histories[i + 1];
        diffs.push(next - history)
    }
    diffs
}

fn process(history: &Vec<i32>) -> i32 {
    let mut all_diffs: Vec<Vec<i32>> = vec![];
    let mut diffs = history.clone();

    all_diffs.push(diffs.clone());

    while diffs.iter().map(|x| *x).sum::<i32>() != 0 {
        diffs = get_diffs(diffs);
        all_diffs.push(diffs.clone());
    }
    all_diffs.iter().flat_map(|x| x.iter().rev().take(1)).sum()
}

fn main() {
    let input = std::fs::read_to_string("day9.txt").unwrap();
    let histories = parse_input(&input);

    let mut part_one: i32 = 0;
    let mut part_two: i32 = 0;

    for history in histories {
        part_one += process(&history);
        part_two += process(&history.into_iter().rev().collect());
    }
    println!("part 1: {}, part 2: {}", part_one, part_two);
}

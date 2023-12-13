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

fn main() {
    let input = std::fs::read_to_string("day9.txt").unwrap();
    let histories = parse_input(&input);

    let mut part_one: i32 = 0;
    let mut part_two: i32 = 0;

    for history in histories {

        let mut all_diffs: Vec<Vec<i32>> = vec![];
        let mut diffs = history;
        all_diffs.push(diffs.clone());

        while diffs.iter().map(|x| *x).sum::<i32>() != 0 {
            diffs = get_diffs(diffs);
            all_diffs.push(diffs.clone());
        }

        let sum: i32 = all_diffs.iter().flat_map(|x| x.iter().rev().take(1)).sum();
        part_one += sum;

        for i in (0..all_diffs.len()).rev() {
            if i == all_diffs.len() - 1 {
                all_diffs[i].insert(0, 0)
            } else {
                let prev = all_diffs[i+1][0];
                let cur = all_diffs[i][0];
                all_diffs[i].insert(0, cur-prev)
            }
        }
        part_two += all_diffs[0][0];
    }
    println!("part 1: {}, part 2: {}", part_one, part_two);
}

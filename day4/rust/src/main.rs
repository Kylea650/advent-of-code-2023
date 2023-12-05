use std::collections::HashMap;
use std::collections::VecDeque;

fn parse_input(input: &str) -> Vec<HashMap<u32, Vec<u32>>> {
    // parse input into a vector of hashmaps
    // key=0: [winning numbers]
    // key=1: [users numbers]
    input
        .lines()
        .map(|x| {
            let mut cards: HashMap<u32, Vec<u32>> = HashMap::new();

            let mut line = x.split(": ");
            let _ = line.next().unwrap();
            let all_numbers = line.next().unwrap();
            let mut split_nums = all_numbers.split(" | ");

            let winning_nums = split_nums.next().unwrap();
            let parsed_winning_nums = winning_nums
                .split(' ')
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<_>>();
            cards.insert(0, parsed_winning_nums);

            let user_nums = split_nums.next().unwrap();
            let parsed_user_nums = user_nums
                .split(' ')
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<_>>();
            cards.insert(1, parsed_user_nums);

            cards
        })
        .collect::<Vec<_>>()
}

fn process(cards: &[HashMap<u32, Vec<u32>>]) -> (u32, u32) {
    let mut part_one: u32 = 0;
    let mut part_two: u32 = 0;

    // hashmap of {card num: num of winning numbers}
    let mut card_wins: HashMap<u32, u32> = HashMap::new();

    for (i, card) in cards.iter().enumerate() {
        let round: u32 = i as u32 + 1;

        // initialize map with the card number and no. of wins = 0
        card_wins.insert(round, 0);

        let winning_numbers = card.get(&0).unwrap();
        let user_numbers = card.get(&1).unwrap();

        // if our number is in the winning list, add a point to our card_wins map
        for number in user_numbers {
            if winning_numbers.contains(number) {
                *card_wins.entry(round).or_insert(0) += 1;
            }
        }
        let points = card_wins.get(&round).unwrap();

        // if that card number has > 0 wins, increment part_one with 2 ^ number of points - 1
        match points {
            0 => part_one += 0,
            _ => part_one += 2_u32.pow(points - 1),
        }
    }

    // initialise queue with all initial card numbers
    let mut deque: VecDeque<u32> = VecDeque::from({
        let len = cards.len() as u32;
        (1..=len).collect::<Vec<_>>()
    });

    // pop the leftmost card from the queue, increment our counter and append the new cards to the end
    // e.g if card 1 wins cards 2, 3 and 4 we remove card 1 from the left and add 2, 3, 4 to the right
    while !deque.is_empty() {
        if let Some(x) = deque.pop_front() {
            let card = x;
            part_two += 1;

            let start = card + 1;
            let num_wins = card_wins.get(&card).unwrap();
            let end = start + num_wins;

            for i in start..end {
                deque.push_back(i);
            }
        }
    }
    (part_one, part_two)
}

fn main() {
    let input = std::fs::read_to_string("day4.txt").unwrap();
    let cards = parse_input(&input);
    let (part_one, part_two) = process(&cards);

    println!("part 1: {}, part 2: {}", part_one, part_two);
}

use std::collections::HashMap;

fn parse_input(input: String) -> Vec<Vec<Vec<Vec<String>>>> {
    input
        .lines()
        .map(|x| {
            let mut line = x.split(": ");
            let _game = line.next().unwrap();
            let results = line.next().unwrap();
            results
                .split("; ")
                .map(|x| {
                    x.split(", ")
                        .map(|x| x.split(' ').map(|x| x.to_string()).collect::<Vec<_>>())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part_one(games: Vec<Vec<Vec<Vec<String>>>>) -> u32 {
    let max_cubes: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut counter: usize = 0;

    for (i, game) in games.into_iter().enumerate() {
        let mut game_possible = true;

        for rounds in game {
            let mut round_possible: Vec<bool> = vec![];

            for round in rounds {
                let count = round[0].parse::<u32>().unwrap();
                let colour = &round[1];
                if &count > max_cubes.get(colour.as_str()).unwrap() {
                    round_possible.push(false)
                }
            }
            if !round_possible.is_empty() {
                game_possible = false
            }
        }
        if game_possible {
            counter += i + 1;
        }
    }
    counter as u32
}

fn part_two(games: Vec<Vec<Vec<Vec<String>>>>) -> u32 {
    let mut powers: u32 = 0;

    for game in games {
        let mut max_colours: HashMap<&str, u32> =
            HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        for rounds in &game {
            for round in rounds {
                let count = round[0].parse::<u32>().unwrap();
                let colour = round[1].as_str();
                let cur_max = max_colours.get(colour).unwrap();

                if count > *cur_max {
                    max_colours.insert(colour, count);
                }
            }
        }
        powers += max_colours.values().product::<u32>();
    }
    powers
}

fn main() {
    let input = std::fs::read_to_string("day2.txt").unwrap();
    let games = parse_input(input);
    let part_one = part_one(games.clone());
    let part_two = part_two(games);

    println!("part 1: {}, part 2: {}", part_one, part_two)
}

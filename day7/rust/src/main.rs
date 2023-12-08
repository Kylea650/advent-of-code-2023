use std::collections::HashMap;

#[derive(Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Copy, Clone)]
struct Card {
    rank: Rank,
}

#[derive(Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Copy, Clone)]
enum Rank {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

impl Rank {
    fn from(c: char) -> Result<Rank, ()> {
        match c {
            '2' => Ok(Rank::Two),
            '3' => Ok(Rank::Three),
            '4' => Ok(Rank::Four),
            '5' => Ok(Rank::Five),
            '6' => Ok(Rank::Six),
            '7' => Ok(Rank::Seven),
            '8' => Ok(Rank::Eight),
            '9' => Ok(Rank::Nine),
            'T' => Ok(Rank::Ten),
            'J' => Ok(Rank::Jack),
            'Q' => Ok(Rank::Queen),
            'K' => Ok(Rank::King),
            'A' => Ok(Rank::Ace),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
enum Hand {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Hand {
    fn from(cards: Vec<Card>) -> Result<Hand, ()> {
        let mut card_counts: HashMap<Card, u8> = HashMap::new();
        for card in cards {
            *card_counts.entry(card).or_insert(0) += 1;
        }
        let mut values: Vec<u8> = card_counts.values().cloned().collect();
        values.sort();

        match values[..] {
            [1, 1, 1, 1, 1] => Ok(Hand::HighCard),
            [1, 1, 1, 2] => Ok(Hand::OnePair),
            [1, 2, 2] => Ok(Hand::TwoPair),
            [1, 1, 3] => Ok(Hand::ThreeOfAKind),
            [2, 3] => Ok(Hand::FullHouse),
            [1, 4] => Ok(Hand::FourOfAKind),
            [5] => Ok(Hand::FiveOfAKind),
            _ => Err(()),
        }
    }
}

fn part_one(input: &str) -> u32 {
    let mut hands: Vec<(Hand, Vec<Card>, u32)> = input
        .lines()
        .map(|x| {
            let mut split = x.split(' ');
            let str_cards = split.next().unwrap().chars();
            let cards: Vec<Card> = str_cards
                .map(|x| {
                    let rank = Rank::from(x).unwrap();
                    Card { rank }
                })
                .collect();
            let hand = Hand::from(cards.clone()).unwrap();
            let bid = split.next().unwrap().parse::<u32>().unwrap();

            (hand, cards, bid)
        })
        .collect();

    hands.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid))| {
            let rank = i + 1;
            rank as u32 * bid
        })
        .sum()
}

#[derive(Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Copy, Clone)]
struct RevisedCard {
    rank: RevisedRank,
}

#[derive(Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Copy, Clone)]
enum RevisedRank {
    Joker = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

impl RevisedRank {
    fn from(c: char) -> Result<RevisedRank, ()> {
        match c {
            '2' => Ok(RevisedRank::Two),
            '3' => Ok(RevisedRank::Three),
            '4' => Ok(RevisedRank::Four),
            '5' => Ok(RevisedRank::Five),
            '6' => Ok(RevisedRank::Six),
            '7' => Ok(RevisedRank::Seven),
            '8' => Ok(RevisedRank::Eight),
            '9' => Ok(RevisedRank::Nine),
            'T' => Ok(RevisedRank::Ten),
            'J' => Ok(RevisedRank::Joker),
            'Q' => Ok(RevisedRank::Queen),
            'K' => Ok(RevisedRank::King),
            'A' => Ok(RevisedRank::Ace),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
enum RevisedHand {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl RevisedHand {
    fn from(cards: Vec<RevisedCard>) -> Result<RevisedHand, ()> {
        let mut card_counts: HashMap<RevisedCard, u8> = HashMap::new();
        for card in cards {
            *card_counts.entry(card).or_insert(0) += 1;
        }
        let mut values: Vec<u8> = card_counts.values().cloned().collect();
        values.sort();

        match values[..] {
            [1, 1, 1, 1, 1] => Ok(RevisedHand::HighCard),
            [1, 1, 1, 2] => Ok(RevisedHand::OnePair),
            [1, 2, 2] => Ok(RevisedHand::TwoPair),
            [1, 1, 3] => Ok(RevisedHand::ThreeOfAKind),
            [2, 3] => Ok(RevisedHand::FullHouse),
            [1, 4] => Ok(RevisedHand::FourOfAKind),
            [5] => Ok(RevisedHand::FiveOfAKind),
            _ => Err(()),
        }
    }

    fn from_with_joker(cards: Vec<RevisedCard>) -> Result<RevisedHand, ()> {
        let mut card_counts: HashMap<RevisedCard, u8> = HashMap::new();
        let mut jokers: u32 = 0;

        for card in cards {
            match card.rank {
                RevisedRank::Joker => jokers += 1,
                _ => *card_counts.entry(card).or_insert(0) += 1,
            }
        }

        let mut values: Vec<u8> = card_counts.values().cloned().collect();
        values.sort();

        match jokers {
            5 => Ok(RevisedHand::FiveOfAKind),
            4 => Ok(RevisedHand::FiveOfAKind),
            3 => match values[..] {
                [1, 1] => Ok(RevisedHand::FourOfAKind),
                [2] => Ok(RevisedHand::FiveOfAKind),
                _ => Err(()),
            },
            2 => match values[..] {
                [1, 1, 1] => Ok(RevisedHand::ThreeOfAKind),
                [1, 2] => Ok(RevisedHand::FourOfAKind),
                [3] => Ok(RevisedHand::FiveOfAKind),
                _ => Err(()),
            },
            1 => match values[..] {
                [1, 1, 1, 1] => Ok(RevisedHand::OnePair),
                [1, 1, 2] => Ok(RevisedHand::ThreeOfAKind),
                [2, 2] => Ok(RevisedHand::FullHouse),
                [1, 3] => Ok(RevisedHand::FourOfAKind),
                [4] => Ok(RevisedHand::FiveOfAKind),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

fn contains_joker(cards: &[RevisedCard]) -> bool {
    for card in cards {
        match card.rank {
            RevisedRank::Joker => return true,
            _ => continue,
        }
    }
    false
}

fn part_two(input: &str) -> u32 {
    let mut hands: Vec<(RevisedHand, Vec<RevisedCard>, u32)> = input
        .lines()
        .map(|x| {
            let mut split = x.split(' ');
            let str_cards = split.next().unwrap().chars();
            let cards: Vec<RevisedCard> = str_cards
                .map(|x| {
                    let rank = RevisedRank::from(x).unwrap();
                    RevisedCard { rank }
                })
                .collect();

            let hand = match contains_joker(&cards) {
                true => RevisedHand::from_with_joker(cards.clone()).unwrap(),
                false => RevisedHand::from(cards.clone()).unwrap(),
            };
            let bid = split.next().unwrap().parse::<u32>().unwrap();

            (hand, cards, bid)
        })
        .collect();

    hands.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid))| {
            let rank = i + 1;
            rank as u32 * bid
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("day7.txt").unwrap();
    let part_one = part_one(&input);
    let part_two = part_two(&input);

    println!("part 1: {}, part 2: {}", part_one, part_two)
}

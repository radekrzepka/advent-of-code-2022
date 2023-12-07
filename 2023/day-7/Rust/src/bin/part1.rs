use std::fs;
use std::io::{self};

const CARD_VALUES: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 1,
    FourOfAKind = 2,
    FullHouse = 3,
    ThreeOfAKind = 4,
    TwoPair = 5,
    OnePair = 6,
    HighCard = 7
}

#[derive(Debug)]
struct Hand {
    cards: [char; 5],
    bid: i32,
    hand_type: HandType
}

impl Hand {
    fn group_cards (cards: [char; 5]) -> Vec<Vec<char>> {
        let mut groups: Vec<Vec<char>> = Vec::new();
        let mut current_group: Vec<char> = Vec::new();

        current_group.push(*cards.first().unwrap());

        for &card in cards.iter().skip(1) {
            if let Some(&last_card) = current_group.last() {
                if card == last_card {
                    current_group.push(card);
                } else {
                    groups.push(current_group);
                    current_group = vec![card];
                }
            }
        }

        if !current_group.is_empty() {
            groups.push(current_group);
        }

        groups
    }

    fn has_group_of_size(groups: &[Vec<char>], size: usize) -> i32 {
        return groups.iter().filter(|group| group.len() == size).count() as i32;
    }

    fn generate_type (cards: [char; 5]) -> HandType {
        let groups = Hand::group_cards(cards);

        if Hand::has_group_of_size(&groups, 5) == 1 {return HandType::FiveOfAKind}
        if Hand::has_group_of_size(&groups, 4) == 1 {return HandType::FourOfAKind}
        if Hand::has_group_of_size(&groups, 3) == 1 && Hand::has_group_of_size(&groups, 2) == 1 {return HandType::FullHouse}
        if Hand::has_group_of_size(&groups, 3) == 1 {return HandType::ThreeOfAKind}
        if Hand::has_group_of_size(&groups, 2) == 2 {return HandType::TwoPair}
        if Hand::has_group_of_size(&groups, 2) == 1 {return HandType::OnePair}

        return HandType::HighCard;
    }

    pub fn new(cards: [char; 5], bid: i32) -> Self {
        let mut sorted_cards = cards.clone();
        sorted_cards.sort_by(|&a, &b| {
            let res1 = CARD_VALUES.iter().position(|&x| x == a).unwrap() as i32;
            let res2 = CARD_VALUES.iter().position(|&x| x == b).unwrap() as i32;
            return res1.cmp(&res2);
        });
        let hand_type = Hand::generate_type(sorted_cards);

        Hand {cards, bid, hand_type}
    }
}

pub fn main() -> io::Result<()> {
    let file = fs::read_to_string("./src/inputs/input.txt").unwrap();
    let lines = file.split("\r\n");
    let mut hands: Vec<Hand> = Vec::new();
    let mut result = 0;

    for line in lines {
        let mut split = line.split_ascii_whitespace();

        let cards: [char; 5] = split.next().unwrap().chars().collect::<Vec<char>>().try_into().unwrap();
        let bid = split.next().unwrap().parse::<i32>().unwrap();
        let hand = Hand::new(cards, bid);

        hands.push(hand);
    }

    hands.sort_by(|a, b| {
        match a.hand_type.cmp(&b.hand_type) {
            std::cmp::Ordering::Equal => {
                for (card_a, card_b) in a.cards.iter().zip(b.cards.iter()) {
                    let res1 = CARD_VALUES.iter().position(|&x| x == *card_a).unwrap() as i32;
                    let res2 = CARD_VALUES.iter().position(|&x| x == *card_b).unwrap() as i32;
                    match res1.cmp(&res2) {
                        std::cmp::Ordering::Equal => continue,
                        non_equal => return non_equal,
                    }
                }
                std::cmp::Ordering::Equal
            },
            other => other,
        }
    });

    for (i, hand) in hands.into_iter().rev().enumerate() {
        println!("{:?}", hand);
        result += (i + 1) as i32 * hand.bid;
    }

    println!("{}", result);

    Ok(())
}
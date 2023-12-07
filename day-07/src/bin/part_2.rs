use std::cmp::Ordering;
use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CardType {
    None,
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, Clone)]
struct Hand {
    bid: u32,
    cards: String,
    cards_hash: BTreeMap<char, u32>,
    type_: CardType,
}

#[derive(Debug, Clone)]
struct Game {
    hands: Vec<Hand>,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.type_ == other.type_ && self.cards == other.cards
    }
}

impl Eq for Hand {}

fn custom_card_order(c: char) -> i32 {
    match c {
        'A' => 1,
        'K' => 2,
        'Q' => 3,
        'T' => 4,
        '9' => 5,
        '8' => 6,
        '7' => 7,
        '6' => 8,
        '5' => 9,
        '4' => 10,
        '3' => 11,
        '2' => 12,
        'J' => 13,
        _ => 14,
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.type_.cmp(&other.type_) {
            Ordering::Equal => self
                .cards
                .chars()
                .zip(other.cards.chars())
                .find_map(|(a, b)| {
                    let order = custom_card_order(a).cmp(&custom_card_order(b)).reverse();
                    if order == Ordering::Equal {
                        None
                    } else {
                        Some(order)
                    }
                })
                .or(Some(Ordering::Equal)),
            other => Some(other),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.type_.cmp(&other.type_) {
            Ordering::Equal => self
                .cards
                .chars()
                .zip(other.cards.chars())
                .find_map(|(a, b)| {
                    let order = custom_card_order(a).cmp(&custom_card_order(b)).reverse();
                    if order == Ordering::Equal {
                        None
                    } else {
                        Some(order)
                    }
                })
                .unwrap_or(Ordering::Equal),
            other => other,
        }
    }
}

impl Hand {
    fn new(cards: String, bid: u32) -> Self {
        let mut hash = BTreeMap::new();
        for card in cards.chars() {
            let count = hash.entry(card).or_insert(0);
            *count += 1;
        }

        let mut hand = Hand {
            bid,
            cards,
            cards_hash: hash,
            type_: CardType::None,
        };

        hand.card_type();

        hand
    }

    fn is_five_kind(&self) -> bool {
        self.cards_hash.values().any(|&v| v == 5)
    }

    fn is_four_kind(&self) -> bool {
        self.cards_hash.values().any(|&v| v == 4)
    }

    fn is_full_house(&self) -> bool {
        self.cards_hash.values().any(|&v| v == 3) && self.cards_hash.values().any(|&v| v == 2)
    }

    fn is_three_kind(&self) -> bool {
        self.cards_hash.values().any(|&v| v == 3) && self.cards_hash.values().any(|&v| v == 1)
    }

    fn is_two_pair(&self) -> bool {
        self.cards_hash.values().filter(|&v| *v == 2).count() == 2
    }

    fn is_one_pair(&self) -> bool {
        self.cards_hash.values().any(|&v| v == 2) && self.cards_hash.values().any(|&v| v == 1)
    }

    fn is_high_card(&self) -> bool {
        self.cards_hash.values().all(|&v| v == 1)
    }

    fn find_best_type(&mut self) {
        let n_j = self.cards_hash.keys().filter(|&k| *k == 'J').count();
    }

    fn card_type(&mut self) {
        if self.is_five_kind() {
            self.type_ = CardType::FiveKind
        } else if self.is_four_kind() {
            self.type_ = CardType::FourKind
        } else if self.is_full_house() {
            self.type_ = CardType::FullHouse
        } else if self.is_three_kind() {
            self.type_ = CardType::ThreeKind
        } else if self.is_two_pair() {
            self.type_ = CardType::TwoPair
        } else if self.is_one_pair() {
            self.type_ = CardType::OnePair
        } else if self.is_high_card() {
            self.type_ = CardType::HighCard
        }
    }
}

impl Game {
    fn new() -> Self {
        Self { hands: Vec::new() }
    }
    fn parse_input(&mut self, input: &str) {
        let hands: Vec<Hand> = input
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(" ").unwrap();
                Hand::new(hand.to_string(), bid.parse().unwrap())
            })
            .collect();

        self.hands = hands;
    }
}

fn part_1(input: &str) -> u32 {
    let mut game = Game::new();
    game.parse_input(input);

    game.hands.sort();

    game.hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_output() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        let result = part_1(input);
        assert_eq!(result, 6440)
    }
}

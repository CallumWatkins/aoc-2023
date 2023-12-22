use static_assertions::const_assert_eq;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let lines = read_lines("input.txt");
    let mut hands: Vec<Hand> = lines.map_while(|l| l.parse::<Hand>().ok()).collect();
    hands.sort_unstable();
    let total_winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, h)| acc + ((i + 1) * h.bet as usize));

    println!("Total winnings: {total_winnings}");
}

fn read_lines<P>(filename: P) -> Box<dyn Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    Box::new(
        io::BufReader::new(file)
            .lines()
            .map(std::result::Result::unwrap),
    )
}

const CARD_COUNT: usize = 13;
const HAND_SIZE: usize = 5;
type Card = u8;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandRank {
    fn from(cards: [Card; HAND_SIZE]) -> Self {
        const_assert_eq!(HAND_SIZE, 5); // This method assumes a hand size of 5
        let mut quantities = [0u8; CARD_COUNT];
        for card in cards {
            quantities[card as usize] += 1;
        }
        let mut rank = Self::HighCard;
        for q in quantities.iter().skip(1) {
            rank = match (rank, q) {
                (_, 5) => Self::FiveOfAKind,
                (_, 4) => Self::FourOfAKind,
                (Self::Pair, 3) | (Self::ThreeOfAKind, 2) => Self::FullHouse,
                (_, 3) => Self::ThreeOfAKind,
                (Self::Pair, 2) => Self::TwoPair,
                (_, 2) => Self::Pair,
                (r, _) => r,
            }
        }
        let jokers = quantities[0];
        rank = match (rank, jokers) {
            (_, 5 | 4) | (Self::Pair, 3) | (Self::ThreeOfAKind, 2) | (Self::FourOfAKind, 1) => {
                Self::FiveOfAKind
            }
            (_, 3) | (Self::Pair, 2) | (Self::ThreeOfAKind, 1) => Self::FourOfAKind,
            (_, 2) | (Self::Pair, 1) => Self::ThreeOfAKind,
            (Self::TwoPair, 1) => Self::FullHouse,
            (_, 1) => Self::Pair,
            (r, 0) => r,
            (_, j) => panic!("{j} joker(s) unused"),
        };
        rank
    }
}

impl PartialEq for HandRank {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for HandRank {}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandRank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; HAND_SIZE],
    bet: u16,
    rank: HandRank,
}

impl Hand {
    fn new(cards: [Card; HAND_SIZE], bet: u16) -> Self {
        Hand {
            cards,
            bet,
            rank: HandRank::from(cards),
        }
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let mut cards: [Card; HAND_SIZE] = [0; HAND_SIZE];
        let mut hand_parts = s.split_whitespace();
        let mut hand_cards = hand_parts.next().expect("has hand").chars();
        let bet = hand_parts
            .next()
            .expect("has bet")
            .parse::<u16>()
            .expect("bet is valid u16");

        for card in &mut cards {
            *card = match hand_cards.next().expect("has card") {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'T' => 9,
                '9' => 8,
                '8' => 7,
                '7' => 6,
                '6' => 5,
                '5' => 4,
                '4' => 3,
                '3' => 2,
                '2' => 1,
                'J' => 0,
                _ => panic!("invalid card"),
            }
        }

        Hand::new(cards, bet)
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand::from(s))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.rank.cmp(&other.rank) {
            std::cmp::Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(s, o)| match s.cmp(o) {
                    std::cmp::Ordering::Equal => None,
                    o => Some(o),
                })
                .unwrap_or(std::cmp::Ordering::Equal),
            o => o,
        }
    }
}

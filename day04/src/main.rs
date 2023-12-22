use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::iter;
use std::path::Path;

fn main() {
    part_1();
    part_2();
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

#[derive(Debug)]
struct Scratchcard {
    winning_numbers: HashSet<u8>,
    card_numbers: HashSet<u8>,
}

impl Scratchcard {
    fn with_capacity(winning_number_count: usize, card_number_count: usize) -> Scratchcard {
        Scratchcard {
            winning_numbers: HashSet::with_capacity(winning_number_count),
            card_numbers: HashSet::with_capacity(card_number_count),
        }
    }

    fn points(&self) -> u32 {
        match self.matching_numbers_count() {
            0 => 0,
            n => u32::pow(2, n - 1),
        }
    }

    fn matching_numbers_count(&self) -> u32 {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .count()
            .try_into()
            .unwrap()
    }
}

fn part_1() {
    let winning_number_count = 10;
    let card_number_count = 25;

    let mut points_sum: u32 = 0;

    for line in read_lines("input.txt") {
        let card = parse_card(&line, winning_number_count, card_number_count);
        points_sum += card.points();
    }

    println!("Sum of scratchcard points: {points_sum}");
}

fn part_2() {
    let winning_number_count = 10;
    let card_number_count = 25;

    let mut cards_sum: u32 = 0;
    let mut card_counts = VecDeque::<u32>::with_capacity(winning_number_count);

    for line in read_lines("input.txt") {
        let card = parse_card(&line, winning_number_count, card_number_count);
        let current_card_count = card_counts.pop_front().unwrap_or(1);
        for i in 0..card.matching_numbers_count() {
            if let Some(v) = card_counts.get_mut(i.try_into().unwrap()) {
                *v += current_card_count;
            } else {
                card_counts.push_back(1 + current_card_count);
            }
        }
        cards_sum += current_card_count;
    }

    println!("Sum of scratchcards: {cards_sum}");
}

fn parse_card(line: &str, winning_number_count: usize, card_number_count: usize) -> Scratchcard {
    let mut card = Scratchcard::with_capacity(winning_number_count, card_number_count);
    let mut chars = line
        .as_bytes()
        .iter()
        .map(|b| *b as char)
        .chain(iter::once(' '))
        .enumerate();

    chars
        .by_ref()
        .take_while(|(_, c)| *c != ':')
        .for_each(|_| {});

    let mut num_start: Option<usize> = None;
    let mut num_end: Option<usize> = None;
    for (i, c) in chars.by_ref() {
        match c {
            '|' => break,
            ' ' if num_start.is_none() => (),
            ' ' => {
                if num_end.is_none() {
                    num_end = num_start;
                }

                let num = &line[num_start.unwrap()..=num_end.unwrap()];
                let n: u8 = num.parse().unwrap();
                card.winning_numbers.insert(n);
                num_start = None;
                num_end = None;
            }
            _ => {
                if num_start.is_none() {
                    num_start = Some(i);
                } else {
                    num_end = Some(i);
                }
            }
        }
    }

    for (i, c) in chars {
        match c {
            ' ' if num_start.is_none() => (),
            ' ' => {
                if num_end.is_none() {
                    num_end = num_start;
                }

                let num = &line[num_start.unwrap()..=num_end.unwrap()];
                let n: u8 = num.parse().unwrap();
                card.card_numbers.insert(n);
                num_start = None;
                num_end = None;
            }
            _ => {
                if num_start.is_none() {
                    num_start = Some(i);
                } else {
                    num_end = Some(i);
                }
            }
        }
    }

    card
}

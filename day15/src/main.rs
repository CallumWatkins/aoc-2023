use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
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

struct InitSequence {
    steps: Box<[Box<[u8]>]>,
}

impl InitSequence {
    fn read() -> Self {
        Self {
            steps: read_lines("input.txt")
                .next()
                .expect("has first line")
                .split(',')
                .map(|s| s.as_bytes().into())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    fn hash_sum(&self) -> usize {
        self.steps
            .iter()
            .map(|step| Self::hash(step) as usize)
            .sum()
    }

    fn hash(chars: &[u8]) -> u8 {
        let mut val = 0u8;
        for &char in chars {
            val = val.wrapping_add(char);
            val = val.wrapping_mul(17);
        }
        val
    }

    fn focusing_power(&self) -> usize {
        let mut boxes: [VecDeque<(&[u8], u8)>; 256] = std::array::from_fn(|_| VecDeque::new());
        for step in self.steps.iter() {
            match step.as_ref() {
                [label @ .., b'-'] => {
                    let b = &mut boxes[Self::hash(label) as usize];
                    if let Some(i) = b.iter().position(|e| e.0.starts_with(label)) {
                        b.remove(i);
                    }
                }
                [label @ .., b'=', focal_length_char] => {
                    let b = &mut boxes[Self::hash(label) as usize];
                    if let Some(slot) = b.iter_mut().find(|e| e.0.starts_with(label)) {
                        *slot = (label, *focal_length_char - 48);
                    } else {
                        b.push_back((label, *focal_length_char - 48));
                    }
                }
                _ => panic!("unexpected step value"),
            }
        }
        boxes
            .iter()
            .enumerate()
            .map(|(box_id, b)| {
                b.iter()
                    .enumerate()
                    .map(|(slot_id, (_label, focal_length))| {
                        (box_id + 1) * (slot_id + 1) * (*focal_length as usize)
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

fn part_1() {
    let seq = InitSequence::read();
    let sum: usize = seq.hash_sum();
    println!("Sum of initialization sequence hashes: {sum}");
}

fn part_2() {
    let seq = InitSequence::read();
    let power: usize = seq.focusing_power();
    println!("Focusing power: {power}");
}

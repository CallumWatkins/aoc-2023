use lru::LruCache;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::once;
use std::num::NonZeroUsize;
use std::path::Path;
use std::str::FromStr;

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

#[derive(Debug, Copy, Clone)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl From<u8> for Condition {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Operational,
            b'#' => Self::Damaged,
            b'?' => Self::Unknown,
            _ => panic!("invalid condition character"),
        }
    }
}

struct SpringRecord {
    conditions: Box<[Condition]>,
    contiguous: Box<[u8]>,
}

impl FromStr for SpringRecord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl From<&str> for SpringRecord {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();
        SpringRecord {
            conditions: parts
                .next()
                .expect("has condition record")
                .bytes()
                .map(Condition::from)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            contiguous: parts
                .next()
                .expect("has contiguous record")
                .split(',')
                .map(|n| n.parse::<u8>().expect("valid u8"))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
}

impl SpringRecord {
    fn unfold(&mut self) {
        let mut unfolded_conditions = (0..5)
            .flat_map(|_| self.conditions.iter().chain(once(&Condition::Unknown)))
            .copied()
            .collect::<Vec<_>>();
        unfolded_conditions.pop().unwrap();
        self.conditions = unfolded_conditions.into_boxed_slice();

        self.contiguous = (0..5)
            .flat_map(|_| self.contiguous.as_ref())
            .copied()
            .collect::<Vec<_>>()
            .into_boxed_slice();
    }

    fn arrangements(&self) -> usize {
        let mut cache: LruCache<(usize, usize), usize> =
            LruCache::new(NonZeroUsize::new(100).unwrap());

        Self::arrangements_rec(&self.conditions, &self.contiguous, 0, &mut cache)
    }

    fn arrangements_rec(
        conditions: &[Condition],
        contiguous_blocks: &[u8],
        offset: usize,
        cache: &mut LruCache<(usize, usize), usize>,
    ) -> usize {
        if contiguous_blocks.is_empty() {
            // No more items, base case
            return 1;
        }
        if let Some(&cached_result) = cache.get(&(contiguous_blocks.len(), offset)) {
            return cached_result;
        }
        let is_last_contiguous_block = contiguous_blocks.len() > 1;
        let contiguous_block = contiguous_blocks[0];
        let required_len = if is_last_contiguous_block {
            contiguous_block + 1
        } else {
            contiguous_block
        } as usize;

        let remaining_space_required = contiguous_blocks
            .iter()
            .skip(1)
            .fold(0, |acc, x| acc + x + 1)
            .saturating_sub(1) as usize;

        (0..=conditions
            .len()
            .saturating_sub(offset)
            .saturating_sub(required_len)
            .saturating_sub(remaining_space_required))
            .map(|position| {
                let skipped = &conditions[offset..offset + position];
                if skipped.iter().any(|c| matches!(c, Condition::Damaged)) {
                    // Damaged spring was skipped
                    return 0;
                }

                let placement =
                    &conditions[offset + position..offset + position + contiguous_block as usize];
                if placement
                    .iter()
                    .any(|c| matches!(c, Condition::Operational))
                {
                    // Position contains operational springs
                    return 0;
                }

                if is_last_contiguous_block {
                    if matches!(
                        conditions[offset + position + required_len - 1],
                        Condition::Damaged
                    ) {
                        // No room for operational spacer after placement
                        return 0;
                    }
                } else {
                    let remaining = &conditions[offset + position + contiguous_block as usize..];
                    if remaining.iter().any(|c| matches!(c, Condition::Damaged)) {
                        // Damaged spring left over
                        return 0;
                    }
                }

                let child_arrangements = Self::arrangements_rec(
                    conditions,
                    &contiguous_blocks[1..],
                    offset + position + required_len,
                    cache,
                );
                cache.put(
                    (
                        contiguous_blocks.len() - 1,
                        offset + position + required_len,
                    ),
                    child_arrangements,
                );
                child_arrangements
            })
            .sum()
    }
}

fn part_1() {
    let sum: usize = read_lines("input.txt")
        .map(|l| l.parse::<SpringRecord>().expect("valid row").arrangements())
        .sum();

    println!("Sum of operational arrangements: {sum}");
}

fn part_2() {
    let sum: usize = read_lines("input.txt")
        .map(|l| {
            let mut record = l.parse::<SpringRecord>().expect("valid row");
            record.unfold();
            record.arrangements()
        })
        .sum();

    println!("Sum of operational arrangements (unfolded): {sum}");
}

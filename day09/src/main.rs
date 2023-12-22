use std::fs::File;
use std::io::{self, BufRead};
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

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct ValueReport {
    history: Box<[i32]>,
}

impl ValueReport {
    fn extrapolate(&self) -> i32 {
        let mut col1 = vec![0; self.history.len()];
        let mut col2 = vec![0; self.history.len()];

        col2[0] = self.history[0];

        for (i, history_entry) in self.history.iter().skip(1).enumerate() {
            // copy col2 to col1
            for (c1, c2) in col1.iter_mut().zip(col2.iter()) {
                *c1 = *c2;
            }

            // set col2[0] to current history entry
            col2[0] = *history_entry;

            // calculate difference between col1 and col2 and store in col2
            for j in 1..col2.len().min(i + 2) {
                col2[j] = col2[j - 1] - col1[j - 1];
            }
        }

        col2.iter().sum::<i32>()
    }
}

impl From<&str> for ValueReport {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl FromStr for ValueReport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ValueReport {
            history: s
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        })
    }
}

fn part_1() {
    let extrapolated_value_sum = read_lines("input.txt")
        .map(|line| {
            let value_report: ValueReport = line.parse().expect("valid report line");
            value_report.extrapolate()
        })
        .sum::<i32>();

    println!("Sum of extrapolated values: {extrapolated_value_sum}");
}

fn part_2() {
    let extrapolated_value_sum = read_lines("input.txt")
        .map(|line| {
            let mut value_report: ValueReport = line.parse().expect("valid report line");
            value_report.history.reverse();
            value_report.extrapolate()
        })
        .sum::<i32>();

    println!("Sum of reverse extrapolated values: {extrapolated_value_sum}");
}

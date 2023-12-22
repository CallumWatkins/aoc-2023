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

#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    fn ways_to_win(&self) -> u64 {
        let mut ways = 0;
        for time_held in 1..self.time {
            let time_remaining = self.time - time_held;
            if u128::from(time_remaining) * u128::from(time_held) > u128::from(self.record_distance)
            {
                ways += 1;
            }
        }
        ways
    }
}

fn part_1() {
    let mut races: Vec<Race> = Vec::new();
    let mut lines = read_lines("input.txt");
    let times_str = lines.next().expect("has first line");
    let times = times_str.split_whitespace().skip(1);
    let distances_str = lines.next().expect("has second line");
    let distances = distances_str.split_whitespace().skip(1);

    times.zip(distances).for_each(|(t, d)| {
        races.push(Race {
            time: t.parse::<u64>().expect("time is valid u64"),
            record_distance: d.parse::<u64>().expect("distance is valid u64"),
        });
    });

    let product = races.iter().map(Race::ways_to_win).product::<u64>();

    println!("Product of ways to win: {product}");
}

fn part_2() {
    let mut lines = read_lines("input.txt");
    let time_str = lines.next().expect("has first line");
    let time = time_str
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |acc, e| acc + e)
        .parse::<u64>()
        .expect("time is valid u64");
    let distance_str = lines.next().expect("has second line");
    let distance = distance_str
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |acc, e| acc + e)
        .parse::<u64>()
        .expect("distance is valid u64");

    let race = Race {
        time,
        record_distance: distance,
    };

    let ways = race.ways_to_win();
    println!("Ways to win: {ways}");
}

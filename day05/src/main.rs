use rayon::prelude::*;
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
struct AlmanacMapEntry {
    dst_range_start: u32,
    src_range_start: u32,
    range_len: u32,
}

impl AlmanacMapEntry {
    fn map(&self, input: u32) -> Option<u32> {
        if input < self.src_range_start || input - self.src_range_start >= self.range_len {
            None
        } else {
            Some(self.dst_range_start + (input - self.src_range_start))
        }
    }
}

#[derive(Debug)]
struct AlmanacMap {
    name: String,
    entries: Vec<AlmanacMapEntry>,
}

impl AlmanacMap {
    fn new() -> Self {
        AlmanacMap {
            name: String::new(),
            entries: Vec::new(),
        }
    }

    fn map(&self, input: u32) -> Option<u32> {
        for entry in &self.entries {
            let output = entry.map(input);
            if output.is_some() {
                return output;
            }
        }
        None
    }
}

struct Seeds {
    seeds: Vec<(u32, u32)>,
}

impl Seeds {
    fn new() -> Self {
        Seeds { seeds: Vec::new() }
    }

    fn map_all(&mut self, map: &AlmanacMap) {
        for seed in &mut self.seeds {
            if let Some(output) = map.map(seed.1) {
                seed.1 = output;
            }
        }
    }
}

fn part_1() {
    let mut seeds = Seeds::new();
    let mut almanac_map = AlmanacMap::new();

    let mut lines = read_lines("input.txt");
    seeds.seeds.extend(
        lines
            .next()
            .expect("has first line")
            .split(' ')
            .skip(1)
            .map(|s| s.parse::<u32>().expect("seed is a valid u32"))
            .map(|s| (s, s)),
    );
    lines.next();

    let mut new_map_name: Option<String> = None;
    loop {
        match lines.next() {
            None => {
                seeds.map_all(&almanac_map);
                break;
            }
            Some(line) if line.is_empty() => seeds.map_all(&almanac_map),
            Some(line) if line.ends_with(':') => {
                new_map_name = Some(line[0..line.len() - 5].into());
            }
            Some(line) => {
                if let Some(new_name) = new_map_name {
                    almanac_map.entries.clear();
                    almanac_map.name = new_name;
                    new_map_name = None;
                }
                let mut map_values = line
                    .split(' ')
                    .map(|s| s.parse::<u32>().expect("seed is a valid u32"));
                let almanac_map_entry: AlmanacMapEntry = AlmanacMapEntry {
                    dst_range_start: map_values.next().expect("has dst_range_start"),
                    src_range_start: map_values.next().expect("has src_range_start"),
                    range_len: map_values.next().expect("has range_len"),
                };
                almanac_map.entries.push(almanac_map_entry);
            }
        }
    }

    let min_location_number = seeds
        .seeds
        .into_iter()
        .min_by_key(|e| e.1)
        .expect("seeds is not empty")
        .1;

    println!("Min location number: {min_location_number}");
}

fn part_2() {
    let mut almanac_map: Option<AlmanacMap> = None;
    let mut almanac_maps: Vec<AlmanacMap> = Vec::new();

    let mut lines = read_lines("input.txt");
    let first_line = lines.next().expect("has first line");
    let seed_numbers = first_line
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().expect("seed is a valid u32"));

    lines.next();

    loop {
        match lines.next() {
            None => {
                almanac_maps.push(almanac_map.take().unwrap());
                break;
            }
            Some(line) if line.is_empty() => {
                almanac_maps.push(almanac_map.take().unwrap());
                almanac_map = None;
            }
            Some(line) if line.ends_with(':') => {
                let mut new_map = AlmanacMap::new();
                new_map.name = line[0..line.len() - 5].into();
                almanac_map = Some(new_map);
            }
            Some(line) => {
                let mut map_values = line
                    .split(' ')
                    .map(|s| s.parse::<u32>().expect("seed is a valid u32"));
                let almanac_map_entry: AlmanacMapEntry = AlmanacMapEntry {
                    dst_range_start: map_values.next().expect("has dst_range_start"),
                    src_range_start: map_values.next().expect("has src_range_start"),
                    range_len: map_values.next().expect("has range_len"),
                };
                almanac_map
                    .as_mut()
                    .unwrap()
                    .entries
                    .push(almanac_map_entry);
            }
        }
    }

    let min_location_number = seed_numbers
        .clone()
        .step_by(2)
        .zip(seed_numbers.skip(1).step_by(2))
        .par_bridge()
        .flat_map(|(a, b)| (a..a + b))
        .map(|mut s| {
            for m in &almanac_maps {
                s = m.map(s).unwrap_or(s);
            }
            s
        })
        .min()
        .expect("result is not empty");

    println!("Min location number: {min_location_number}");
}

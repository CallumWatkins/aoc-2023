use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    part_1()?;
    part_2()?;

    Ok(())
}

fn part_1() -> Result<(), Box<dyn Error>> {
    let mut sum: u32 = 0;

    for line in read_lines("input.txt") {
        let mut first_digit: Option<u8> = None;
        let mut last_digit: Option<u8> = None;

        let mut i = 0;
        while first_digit.is_none() && i < line.len() {
            let slice = line.get(i..).unwrap();
            if let Some(n) = starting_digit(slice) {
                first_digit = Some(n);
                break;
            }
            i += 1;
        }

        if first_digit.is_none() {
            Err(format!("No numbers found in line '{line}'"))?;
        }

        let mut j: usize = line.len() - 1;
        while j >= i {
            if let Some(n) = starting_digit(line.get(j..).unwrap()) {
                last_digit = Some(n);
                break;
            }
            j -= 1;
        }

        sum += u32::from(first_digit.unwrap()) * 10;
        sum += u32::from(last_digit.unwrap());
    }

    println!("Calibration sum: {sum}");
    Ok(())
}

fn starting_digit(slice: &str) -> Option<u8> {
    let c = slice.as_bytes()[0];
    if c.is_ascii_digit() {
        return Some(c - 48);
    }
    None
}

fn part_2() -> Result<(), Box<dyn Error>> {
    let mut sum: u32 = 0;

    for line in read_lines("input.txt") {
        let mut first_digit: Option<u8> = None;
        let mut last_digit: Option<u8> = None;

        let mut i = 0;
        while first_digit.is_none() && i < line.len() {
            let slice = line.get(i..).unwrap();
            if let Some(n) = starting_number(slice) {
                first_digit = Some(n);
                break;
            }
            i += 1;
        }

        if first_digit.is_none() {
            Err(format!("No numbers found in line '{line}'"))?;
        }

        let mut j: usize = line.len() - 1;
        while j >= i {
            if let Some(n) = starting_number(line.get(j..).unwrap()) {
                last_digit = Some(n);
                break;
            }
            j -= 1;
        }

        sum += u32::from(first_digit.unwrap()) * 10;
        sum += u32::from(last_digit.unwrap());
    }

    println!("Calibration sum (inc. words): {sum}");
    Ok(())
}

fn starting_number(slice: &str) -> Option<u8> {
    let c = slice.as_bytes()[0];
    if c.is_ascii_digit() {
        return Some(c - 48);
    }
    for (i, &word) in [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    {
        if slice.starts_with(word) {
            return Some(u8::try_from(i).unwrap() + 1);
        }
    }

    None
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

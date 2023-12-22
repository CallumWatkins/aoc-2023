use std::cmp::min;
use std::collections::HashMap;
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

fn part_1() {
    let mut parts_sum = 0;
    let mut lines = read_lines("input.txt");
    let mut prev_line: Option<Vec<u8>> = None;
    let mut current_line: Option<Vec<u8>> = lines.next().map(|s| s.as_bytes().to_vec());
    let mut next_line: Option<Vec<u8>> = lines.next().map(|s| s.as_bytes().to_vec());

    while current_line.is_some() {
        let mut number_start_index: Option<usize> = None;
        let mut number_end_index: Option<usize> = None;
        let mut number_found = false;
        for (current_line_byte_index, current_line_byte) in current_line
            .as_ref()
            .unwrap()
            .iter()
            .chain(&[b'.'])
            .enumerate()
        {
            match (
                current_line_byte.is_ascii_digit(),
                number_start_index,
                number_end_index,
            ) {
                (true, None, _) => number_start_index = Some(current_line_byte_index),
                (true, Some(_), _) => number_end_index = Some(current_line_byte_index),
                (false, None, _) => (),
                (false, Some(_), None) => {
                    number_end_index = number_start_index;
                    number_found = true;
                }
                (false, Some(_), Some(_)) => {
                    number_found = true;
                }
            }

            if number_found {
                if adjacent_to_symbol(
                    prev_line.as_ref(),
                    current_line.as_ref().unwrap(),
                    next_line.as_ref(),
                    number_start_index.unwrap(),
                    number_end_index.unwrap(),
                ) {
                    let number_slice = &current_line.as_ref().unwrap()
                        [number_start_index.unwrap()..=number_end_index.unwrap()];
                    let mut m: u32 = 1;
                    for digit in number_slice.iter().rev() {
                        parts_sum += u32::from(digit - 48) * m;
                        m *= 10;
                    }
                }
                number_start_index = None;
                number_end_index = None;
                number_found = false;
            }
        }

        prev_line = current_line;
        current_line = next_line;
        next_line = lines.next().map(|s| s.as_bytes().to_vec());
    }

    println!("Sum of engine schematic part numbers: {parts_sum}");
}

fn adjacent_to_symbol(
    prev_line: Option<&Vec<u8>>,
    current_line: &Vec<u8>,
    next_line: Option<&Vec<u8>>,
    number_start_index: usize,
    number_end_index: usize,
) -> bool {
    if number_start_index != 0 && is_symbol(current_line[number_start_index - 1]) {
        return true;
    }

    if number_end_index != current_line.len() - 1 && is_symbol(current_line[number_end_index + 1]) {
        return true;
    }

    let adjacent_search_start_index = number_start_index.saturating_sub(1);
    let adjacent_search_end_index = min(number_end_index + 1, current_line.len() - 1);
    for adjacent_line in [prev_line, next_line].into_iter().flatten() {
        for b in &adjacent_line[adjacent_search_start_index..=adjacent_search_end_index] {
            if is_symbol(*b) {
                return true;
            }
        }
    }
    false
}

fn is_symbol(byte: u8) -> bool {
    !byte.is_ascii_digit() && byte != b'.'
}

fn is_gear(byte: u8) -> bool {
    byte == b'*'
}

fn adjacent_gears(
    prev_line: Option<&Vec<u8>>,
    current_line: &Vec<u8>,
    next_line: Option<&Vec<u8>>,
    number_start_index: usize,
    number_end_index: usize,
    current_line_number: usize,
) -> Vec<(usize, usize)> {
    let mut gears: Vec<(usize, usize)> = vec![];

    if number_start_index != 0 && is_gear(current_line[number_start_index - 1]) {
        gears.push((current_line_number, number_start_index - 1));
    }

    if number_end_index != current_line.len() - 1 && is_gear(current_line[number_end_index + 1]) {
        gears.push((current_line_number, number_end_index + 1));
    }

    let adjacent_search_start_index = number_start_index.saturating_sub(1);
    let adjacent_search_end_index = min(number_end_index + 1, current_line.len() - 1);
    for (adjacent_line_number, adjacent_line) in [
        (current_line_number.saturating_sub(1), prev_line),
        (current_line_number + 1, next_line),
    ] {
        if let Some(bytes) = adjacent_line {
            for (b_index, &b) in bytes[adjacent_search_start_index..=adjacent_search_end_index]
                .iter()
                .enumerate()
            {
                if is_gear(b) {
                    gears.push((adjacent_line_number, b_index + adjacent_search_start_index));
                }
            }
        }
    }
    gears
}

fn part_2() {
    let mut lines = read_lines("input.txt");
    let mut prev_line: Option<Vec<u8>> = None;
    let mut current_line: Option<Vec<u8>> = lines.next().map(|s| s.as_bytes().to_vec());
    let mut next_line: Option<Vec<u8>> = lines.next().map(|s| s.as_bytes().to_vec());
    let mut current_line_index: usize = 0;

    // (line_index, char_index) => (current_product, adjacent_number_count)
    let mut gears: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    while current_line.is_some() {
        let mut number_start_index: Option<usize> = None;
        let mut number_end_index: Option<usize> = None;
        let mut number_found = false;
        for (current_line_byte_index, current_line_byte) in current_line
            .as_ref()
            .unwrap()
            .iter()
            .chain(&[b'.'])
            .enumerate()
        {
            match (
                current_line_byte.is_ascii_digit(),
                number_start_index,
                number_end_index,
            ) {
                (true, None, _) => number_start_index = Some(current_line_byte_index),
                (true, Some(_), _) => number_end_index = Some(current_line_byte_index),
                (false, None, _) => (),
                (false, Some(_), None) => {
                    number_end_index = number_start_index;
                    number_found = true;
                }
                (false, Some(_), Some(_)) => {
                    number_found = true;
                }
            }

            if number_found {
                let adjacent_gears = adjacent_gears(
                    prev_line.as_ref(),
                    current_line.as_ref().unwrap(),
                    next_line.as_ref(),
                    number_start_index.unwrap(),
                    number_end_index.unwrap(),
                    current_line_index,
                );
                if !adjacent_gears.is_empty() {
                    let mut number: usize = 0;
                    let number_slice = &current_line.as_ref().unwrap()
                        [number_start_index.unwrap()..=number_end_index.unwrap()];
                    let mut m: usize = 1;
                    for digit in number_slice.iter().rev() {
                        number += (digit - 48) as usize * m;
                        m *= 10;
                    }
                    for gear in adjacent_gears {
                        gears
                            .entry(gear)
                            .and_modify(|v| {
                                v.0 *= number;
                                v.1 += 1;
                            })
                            .or_insert((number, 1));
                    }
                }
                number_start_index = None;
                number_end_index = None;
                number_found = false;
            }
        }

        prev_line = current_line;
        current_line = next_line;
        current_line_index += 1;
        next_line = lines.next().map(|s| s.as_bytes().to_vec());
    }

    let gear_ratios_sum = gears
        .values()
        .filter(|v| v.1 == 2)
        .fold(0, |acc, v| acc + v.0);

    println!("Sum of engine schematic gear ratios: {gear_ratios_sum}");
}

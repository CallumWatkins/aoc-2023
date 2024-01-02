#![feature(let_chains)]
use std::cmp::min;
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

struct Grid {
    cells: Box<[Box<[u8]>]>,
}

impl Grid {
    fn find_horizontal_reflection(&self) -> Option<usize> {
        for i in 0..self.cells.len() - 1 {
            if self.cells[i] == self.cells[i + 1] {
                let mut reflection = true;
                for offset in 1..min(i + 1, self.cells.len() - i - 1) {
                    if self.cells[i - offset] != self.cells[i + offset + 1] {
                        reflection = false;
                        break;
                    }
                }
                if reflection {
                    return Some(i);
                }
            }
        }
        None
    }

    fn find_vertical_reflection(&self) -> Option<usize> {
        for j in 0..self.cells[0].len() - 1 {
            let mut possible_reflection = true;
            for i in 0..self.cells.len() {
                if self.cells[i][j] != self.cells[i][j + 1] {
                    possible_reflection = false;
                    break;
                }
            }
            if possible_reflection {
                let mut reflection = true;
                for offset in 1..min(j + 1, self.cells[0].len() - j - 1) {
                    for i in 0..self.cells.len() {
                        if self.cells[i][j - offset] != self.cells[i][j + offset + 1] {
                            reflection = false;
                            break;
                        }
                    }
                    if !reflection {
                        break;
                    }
                }
                if reflection {
                    return Some(j);
                }
            }
        }
        None
    }

    fn find_smudged_horizontal_reflection(&self) -> Option<usize> {
        for i in 0..self.cells.len() - 1 {
            let mut smudges = 0;
            for offset in 0..min(i + 1, self.cells.len() - i - 1) {
                for j in 0..self.cells[0].len() {
                    if self.cells[i - offset][j] != self.cells[i + offset + 1][j] {
                        smudges += 1;
                        if smudges > 1 {
                            break;
                        }
                    }
                }
                if smudges > 1 {
                    break;
                }
            }
            if smudges == 1 {
                return Some(i);
            }
        }
        None
    }

    fn find_smudged_vertical_reflection(&self) -> Option<usize> {
        for j in 0..self.cells[0].len() - 1 {
            let mut smudges = 0;
            for offset in 0..min(j + 1, self.cells[0].len() - j - 1) {
                for i in 0..self.cells.len() {
                    if self.cells[i][j - offset] != self.cells[i][j + offset + 1] {
                        smudges += 1;
                        if smudges > 1 {
                            break;
                        }
                    }
                }
                if smudges > 1 {
                    break;
                }
            }
            if smudges == 1 {
                return Some(j);
            }
        }
        None
    }
}

struct GridIterator {
    lines: Box<dyn Iterator<Item = String>>,
}

impl Iterator for GridIterator {
    type Item = Grid;

    fn next(&mut self) -> Option<Self::Item> {
        let mut grid: Vec<Box<[u8]>> = vec![];
        loop {
            let line = self.lines.next();
            if let Some(l) = line
                && !l.is_empty()
            {
                grid.push(l.into_bytes().into_boxed_slice());
            } else if !grid.is_empty() {
                return Some(Grid {
                    cells: grid.into_boxed_slice(),
                });
            } else {
                return None;
            }
        }
    }
}

fn grids() -> impl Iterator<Item = Grid> {
    GridIterator {
        lines: read_lines("input.txt"),
    }
}

fn part_1() {
    let sum: usize = grids()
        .map(|grid| {
            if let Some(v) = grid.find_vertical_reflection() {
                v + 1
            } else if let Some(h) = grid.find_horizontal_reflection() {
                (h + 1) * 100
            } else {
                0
            }
        })
        .sum();
    println!("Sum of reflection summaries: {sum}");
}

fn part_2() {
    let sum: usize = grids()
        .map(|grid| {
            if let Some(v) = grid.find_smudged_vertical_reflection() {
                v + 1
            } else if let Some(h) = grid.find_smudged_horizontal_reflection() {
                (h + 1) * 100
            } else {
                0
            }
        })
        .sum();
    println!("Sum of smudged reflection summaries: {sum}");
}

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

#[derive(Debug, Copy, Clone)]
struct Location {
    i: usize,
    j: usize,
}

fn distance_sum(empty_row_size: usize, empty_col_size: usize) -> usize {
    let mut empty_rows = 0;
    let mut empty_cols = Vec::<usize>::new();
    let mut galaxies = Vec::<Location>::new();

    read_lines("input.txt").enumerate().for_each(|(i, line)| {
        let mut row_empty = true;
        for (j, byte) in line.bytes().enumerate() {
            match byte {
                b'#' => {
                    row_empty = false;
                    if empty_cols.len() < j + 1 {
                        empty_cols.resize(j + 1, empty_col_size - 1);
                    }
                    empty_cols[j] = 0;
                    galaxies.push(Location {
                        i: i + (empty_rows * (empty_row_size - 1)),
                        j,
                    });
                }
                b'.' => (),
                c => panic!("unexpected character: {c}"),
            };
        }
        if row_empty {
            empty_rows += 1;
        }
    });

    // Fill empty_cols with cumulative sum
    let mut col_empty_acc = 0;
    for col_empty in &mut empty_cols {
        let temp = *col_empty;
        *col_empty += col_empty_acc;
        if temp != 0 {
            col_empty_acc += temp;
        }
    }

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let g1 = galaxies[i];
            let g2 = galaxies[j];
            // Manhattan distance
            sum += g1.i.abs_diff(g2.i) + g1.j.abs_diff(g2.j);
            // Empty rows already included in locations, just add empty columns
            sum += empty_cols[g1.j].abs_diff(empty_cols[g2.j]);
        }
    }
    sum
}

fn part_1() {
    let sum = distance_sum(2, 2);
    println!("Sum of shortest distances (x2 expansion): {sum}");
}

fn part_2() {
    let sum = distance_sum(1_000_000, 1_000_000);
    println!("Sum of shortest distances (x1M expansion): {sum}");
}

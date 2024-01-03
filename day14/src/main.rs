use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::hash::Hasher;
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

#[derive(Hash)]
enum Cell {
    Empty,
    CubeRock,
    RoundRock,
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Empty,
            b'#' => Self::CubeRock,
            b'O' => Self::RoundRock,
            _ => panic!("unexpected cell character"),
        }
    }
}

#[derive(Hash)]
struct Platform {
    cells: Box<[Box<[Cell]>]>,
}

impl Platform {
    fn read() -> Self {
        Self {
            cells: read_lines("input.txt")
                .map(|line| {
                    line.bytes()
                        .map(Cell::from)
                        .collect::<Vec<_>>()
                        .into_boxed_slice()
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    fn tilt_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn tilt_north(&mut self) {
        fn fill_round(cells: &mut [Box<[Cell]>], i: usize, j: usize, count: usize) {
            for offset in 0..count {
                cells[i + offset][j] = Cell::RoundRock;
            }
        }
        let mut round_count: Vec<usize> = vec![0; self.cells[0].len()];

        for i in (0..self.cells.len()).rev() {
            for (j, count) in round_count.iter_mut().enumerate() {
                if i != 0 {
                    match (&self.cells[i][j], count) {
                        (Cell::RoundRock, count) => {
                            self.cells[i][j] = Cell::Empty;
                            *count += 1;
                        }
                        (Cell::CubeRock, count) if *count > 0 => {
                            fill_round(&mut self.cells, i + 1, j, *count);
                            *count = 0;
                        }
                        _ => (),
                    }
                } else if *count > 0 {
                    match self.cells[i][j] {
                        Cell::CubeRock | Cell::RoundRock => {
                            fill_round(&mut self.cells, i + 1, j, *count);
                        }
                        Cell::Empty => {
                            fill_round(&mut self.cells, i, j, *count);
                        }
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        fn fill_round(cells: &mut [Box<[Cell]>], i: usize, j: usize, count: usize) {
            for offset in 0..count {
                cells[i - offset][j] = Cell::RoundRock;
            }
        }
        let mut round_count: Vec<usize> = vec![0; self.cells[0].len()];

        for i in 0..self.cells.len() {
            for (j, count) in round_count.iter_mut().enumerate() {
                if i != self.cells.len() - 1 {
                    match (&self.cells[i][j], count) {
                        (Cell::RoundRock, count) => {
                            self.cells[i][j] = Cell::Empty;
                            *count += 1;
                        }
                        (Cell::CubeRock, count) if *count > 0 => {
                            fill_round(&mut self.cells, i - 1, j, *count);
                            *count = 0;
                        }
                        _ => (),
                    }
                } else if *count > 0 {
                    match self.cells[i][j] {
                        Cell::CubeRock | Cell::RoundRock => {
                            fill_round(&mut self.cells, i - 1, j, *count);
                        }
                        Cell::Empty => {
                            fill_round(&mut self.cells, i, j, *count);
                        }
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        fn fill_round(cells: &mut [Box<[Cell]>], i: usize, j: usize, count: usize) {
            for offset in 0..count {
                cells[i][j + offset] = Cell::RoundRock;
            }
        }

        for i in 0..self.cells.len() {
            let mut round_count = 0;
            for j in (0..self.cells[i].len()).rev() {
                if j != 0 {
                    match (&self.cells[i][j], &mut round_count) {
                        (Cell::RoundRock, count) => {
                            self.cells[i][j] = Cell::Empty;
                            *count += 1;
                        }
                        (Cell::CubeRock, count) if *count > 0 => {
                            fill_round(&mut self.cells, i, j + 1, *count);
                            *count = 0;
                        }
                        _ => (),
                    }
                } else if round_count > 0 {
                    match self.cells[i][j] {
                        Cell::CubeRock | Cell::RoundRock => {
                            fill_round(&mut self.cells, i, j + 1, round_count);
                        }
                        Cell::Empty => {
                            fill_round(&mut self.cells, i, j, round_count);
                        }
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        fn fill_round(cells: &mut [Box<[Cell]>], i: usize, j: usize, count: usize) {
            for offset in 0..count {
                cells[i][j - offset] = Cell::RoundRock;
            }
        }

        for i in 0..self.cells.len() {
            let mut round_count = 0;
            for j in 0..self.cells[i].len() {
                if j != self.cells[i].len() - 1 {
                    match (&self.cells[i][j], &mut round_count) {
                        (Cell::RoundRock, count) => {
                            self.cells[i][j] = Cell::Empty;
                            *count += 1;
                        }
                        (Cell::CubeRock, count) if *count > 0 => {
                            fill_round(&mut self.cells, i, j - 1, *count);
                            *count = 0;
                        }
                        _ => (),
                    }
                } else if round_count > 0 {
                    match self.cells[i][j] {
                        Cell::CubeRock | Cell::RoundRock => {
                            fill_round(&mut self.cells, i, j - 1, round_count);
                        }
                        Cell::Empty => {
                            fill_round(&mut self.cells, i, j, round_count);
                        }
                    }
                }
            }
        }
    }

    fn calculate_north_load(&self) -> usize {
        self.cells
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| row.iter().filter(|c| matches!(c, Cell::RoundRock)).count() * (i + 1))
            .sum()
    }
}

fn part_1() {
    let mut platform = Platform::read();
    platform.tilt_north();
    let load: usize = platform.calculate_north_load();
    println!("Total load on north support beams after north tilt: {load}");
}

fn part_2() {
    let mut platform = Platform::read();
    let required_cycles = 1_000_000_000;
    let mut states: HashMap<u64, usize> = HashMap::new();
    for i in 0..required_cycles {
        platform.tilt_cycle();
        let mut hasher = DefaultHasher::new();
        platform.hash(&mut hasher);
        let hash = hasher.finish();
        if let Some(&v) = states.get(&hash) {
            let remaining_cycles = ((required_cycles - v - 2) % (i - v)) + 1;
            for _ in 0..remaining_cycles {
                platform.tilt_cycle();
            }
            break;
        }
        states.insert(hash, i);
    }

    let load: usize = platform.calculate_north_load();
    println!("Total load on north support beams after 1B cycles: {load}");
}

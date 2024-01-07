use std::collections::HashSet;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileContent {
    Empty,              // .
    MirrorLeft,         // \
    MirrorRight,        // /
    SplitterVertical,   // |
    SplitterHorizontal, // -
}

impl From<u8> for TileContent {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Empty,
            b'\\' => Self::MirrorLeft,
            b'/' => Self::MirrorRight,
            b'|' => Self::SplitterVertical,
            b'-' => Self::SplitterHorizontal,
            _ => panic!("unexpected tile character"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    content: TileContent,
    energized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    i: usize,
    j: usize,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    tiles: Box<[Box<[Tile]>]>,
}

impl Grid {
    fn read() -> Self {
        Self {
            tiles: read_lines("input.txt")
                .map(|line| {
                    line.bytes()
                        .map(|b| Tile {
                            content: b.into(),
                            energized: false,
                        })
                        .collect::<Vec<_>>()
                        .into_boxed_slice()
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    fn energize(&mut self, start: Beam) -> usize {
        let mut energized = 0;
        let mut beam_stack = vec![start];
        let mut seen = HashSet::<Beam>::new();

        while let Some(beam) = beam_stack.pop() {
            if !seen.insert(beam) {
                continue;
            }

            if !self.tiles[beam.i][beam.j].energized {
                self.tiles[beam.i][beam.j].energized = true;
                energized += 1;
            }

            match (
                &self.tiles[beam.i][beam.j].content,
                beam.i,
                beam.j,
                beam.direction,
            ) {
                (TileContent::Empty | TileContent::SplitterVertical, i, j, Direction::North)
                | (TileContent::MirrorLeft, i, j, Direction::West)
                | (TileContent::MirrorRight, i, j, Direction::East) => {
                    if i > 0 {
                        beam_stack.push(Beam {
                            i: i - 1,
                            j,
                            direction: Direction::North,
                        });
                    }
                }
                (TileContent::Empty | TileContent::SplitterHorizontal, i, j, Direction::East)
                | (TileContent::MirrorLeft, i, j, Direction::South)
                | (TileContent::MirrorRight, i, j, Direction::North) => {
                    if j < self.tiles[beam.i].len() - 1 {
                        beam_stack.push(Beam {
                            i,
                            j: j + 1,
                            direction: Direction::East,
                        });
                    }
                }
                (TileContent::Empty | TileContent::SplitterVertical, i, j, Direction::South)
                | (TileContent::MirrorLeft, i, j, Direction::East)
                | (TileContent::MirrorRight, i, j, Direction::West) => {
                    if i < self.tiles.len() - 1 {
                        beam_stack.push(Beam {
                            i: i + 1,
                            j,
                            direction: Direction::South,
                        });
                    }
                }
                (TileContent::Empty | TileContent::SplitterHorizontal, i, j, Direction::West)
                | (TileContent::MirrorLeft, i, j, Direction::North)
                | (TileContent::MirrorRight, i, j, Direction::South) => {
                    if j > 0 {
                        beam_stack.push(Beam {
                            i,
                            j: j - 1,
                            direction: Direction::West,
                        });
                    }
                }
                (TileContent::SplitterVertical, i, j, Direction::East | Direction::West) => {
                    if i > 0 {
                        beam_stack.push(Beam {
                            i: i - 1,
                            j,
                            direction: Direction::North,
                        });
                    }
                    if i < self.tiles.len() - 1 {
                        beam_stack.push(Beam {
                            i: i + 1,
                            j,
                            direction: Direction::South,
                        });
                    }
                }
                (TileContent::SplitterHorizontal, i, j, Direction::North | Direction::South) => {
                    if j > 0 {
                        beam_stack.push(Beam {
                            i,
                            j: j - 1,
                            direction: Direction::West,
                        });
                    }
                    if j < self.tiles[beam.i].len() - 1 {
                        beam_stack.push(Beam {
                            i,
                            j: j + 1,
                            direction: Direction::East,
                        });
                    }
                }
            }
        }
        energized
    }

    fn energized_tiles(&self) -> usize {
        self.tiles
            .iter()
            .map(|row| row.iter().filter(|tile| tile.energized).count())
            .sum()
    }

    fn reset_energized(&mut self) {
        self.tiles.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|tile| {
                tile.energized = false;
            });
        });
    }
}

fn part_1() {
    let mut grid = Grid::read();
    grid.energize(Beam {
        i: 0,
        j: 0,
        direction: Direction::East,
    });
    let energized = grid.energized_tiles();
    println!("Number of energized tiles: {energized}");
}

fn part_2() {
    let mut max = 0;
    let mut grid = Grid::read();
    for i in 0..grid.tiles.len() {
        max = std::cmp::max(
            max,
            grid.energize(Beam {
                i,
                j: 0,
                direction: Direction::East,
            }),
        );
        grid.reset_energized();

        max = std::cmp::max(
            max,
            grid.energize(Beam {
                i,
                j: grid.tiles[i].len() - 1,
                direction: Direction::West,
            }),
        );
        grid.reset_energized();
    }

    for j in 0..grid.tiles[0].len() {
        max = std::cmp::max(
            max,
            grid.energize(Beam {
                i: 0,
                j,
                direction: Direction::South,
            }),
        );
        grid.reset_energized();
        max = std::cmp::max(
            max,
            grid.energize(Beam {
                i: grid.tiles.len() - 1,
                j,
                direction: Direction::North,
            }),
        );
        grid.reset_energized();
    }
    println!("Max number of energized tiles: {max}");
}

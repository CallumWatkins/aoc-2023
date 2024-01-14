use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;

use pathfinding::prelude::astar;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Start,
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node(usize, usize, Direction, usize);

struct City {
    blocks: Box<[Box<[u8]>]>,
}

impl City {
    fn read() -> Self {
        Self {
            blocks: read_lines("input.txt")
                .map(|line| {
                    line.bytes()
                        .map(|b| b - 48)
                        .collect::<Vec<_>>()
                        .into_boxed_slice()
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    fn minimum_path(
        &self,
        min_consecutive: usize,
        max_consecutive: usize,
    ) -> Option<(Vec<Node>, usize)> {
        let start = Node(0, 0, Direction::Start, 0);
        let end: (usize, usize) = (self.blocks.len() - 1, self.blocks[0].len() - 1);
        astar(
            &start,
            |&Node(i, j, direction, consecutive)| {
                let mut successors = Vec::with_capacity(4);
                match direction {
                    Direction::Start => {
                        if i > 0 {
                            successors.push(Node(i - 1, j, Direction::North, 1));
                        }
                        if i < self.blocks.len() - 1 {
                            successors.push(Node(i + 1, j, Direction::South, 1));
                        }
                        if j > 0 {
                            successors.push(Node(i, j - 1, Direction::West, 1));
                        }
                        if j < self.blocks[i].len() - 1 {
                            successors.push(Node(i, j + 1, Direction::East, 1));
                        }
                    }
                    Direction::North => {
                        if consecutive >= min_consecutive {
                            if j > 0 {
                                successors.push(Node(i, j - 1, Direction::West, 1));
                            }
                            if j < self.blocks[i].len() - 1 {
                                successors.push(Node(i, j + 1, Direction::East, 1));
                            }
                        }
                        if consecutive < max_consecutive && i > 0 {
                            successors.push(Node(i - 1, j, Direction::North, consecutive + 1));
                        }
                    }
                    Direction::East => {
                        if consecutive >= min_consecutive {
                            if i > 0 {
                                successors.push(Node(i - 1, j, Direction::North, 1));
                            }
                            if i < self.blocks.len() - 1 {
                                successors.push(Node(i + 1, j, Direction::South, 1));
                            }
                        }
                        if consecutive < max_consecutive && j < self.blocks[i].len() - 1 {
                            successors.push(Node(i, j + 1, Direction::East, consecutive + 1));
                        }
                    }
                    Direction::South => {
                        if consecutive >= min_consecutive {
                            if j > 0 {
                                successors.push(Node(i, j - 1, Direction::West, 1));
                            }
                            if j < self.blocks[i].len() - 1 {
                                successors.push(Node(i, j + 1, Direction::East, 1));
                            }
                        }
                        if consecutive < max_consecutive && i < self.blocks.len() - 1 {
                            successors.push(Node(i + 1, j, Direction::South, consecutive + 1));
                        }
                    }
                    Direction::West => {
                        if consecutive >= min_consecutive {
                            if i > 0 {
                                successors.push(Node(i - 1, j, Direction::North, 1));
                            }
                            if i < self.blocks.len() - 1 {
                                successors.push(Node(i + 1, j, Direction::South, 1));
                            }
                        }
                        if consecutive < max_consecutive && j > 0 {
                            successors.push(Node(i, j - 1, Direction::West, consecutive + 1));
                        }
                    }
                }
                successors
                    .into_iter()
                    .map(|n @ Node(i, j, _, _)| (n, self.blocks[i][j] as usize))
            },
            |&Node(i, j, _, _)| end.0.abs_diff(i) + end.1.abs_diff(j),
            |&Node(i, j, _, _)| i == end.0 && j == end.1,
        )
    }
}

fn part_1() {
    let city = City::read();
    let min_path = city.minimum_path(0, 3).expect("has path");
    println!(
        "Minimum crucible heat loss: {} (in {} steps)",
        min_path.1,
        min_path.0.len()
    );
}

fn part_2() {
    let city = City::read();
    let min_path = city.minimum_path(4, 10).expect("has path");
    println!(
        "Minimum ultra crucible heat loss: {} (in {} steps)",
        min_path.1,
        min_path.0.len()
    );
}

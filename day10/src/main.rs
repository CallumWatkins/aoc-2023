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
struct Location {
    i: usize,
    j: usize,
}

impl Location {
    fn follow_direction(&self, direction: Direction) -> Location {
        match direction {
            Direction::North => Location {
                i: self.i - 1,
                j: self.j,
            },
            Direction::East => Location {
                i: self.i,
                j: self.j + 1,
            },
            Direction::South => Location {
                i: self.i + 1,
                j: self.j,
            },
            Direction::West => Location {
                i: self.i,
                j: self.j - 1,
            },
        }
    }

    fn in_grid(&self, grid: &[Box<[u8]>]) -> u8 {
        grid[self.i][self.j]
    }

    fn in_grid_mut<'a>(&self, grid: &'a mut [Box<[u8]>]) -> &'a mut u8 {
        &mut grid[self.i][self.j]
    }
}
#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

fn find_connecting_pipe_directions(
    lines: &[Box<[u8]>],
    location: Location,
    exclude: Option<Direction>,
) -> Vec<Direction> {
    let mut r = vec![];
    let location_char = &location.in_grid(lines);
    if !matches!(exclude, Some(Direction::North))
        && [b'S', b'|', b'L', b'J'].contains(location_char)
        && location.i > 0
        && [b'S', b'|', b'7', b'F'].contains(&lines[location.i - 1][location.j])
    {
        r.push(Direction::North);
    }
    if !matches!(exclude, Some(Direction::East))
        && [b'S', b'-', b'L', b'F'].contains(location_char)
        && location.j < lines[location.i].len() - 1
        && [b'S', b'-', b'J', b'7'].contains(&lines[location.i][location.j + 1])
    {
        r.push(Direction::East);
    }
    if !matches!(exclude, Some(Direction::South))
        && [b'S', b'|', b'7', b'F'].contains(location_char)
        && location.i < lines.len() - 1
        && [b'S', b'|', b'L', b'J'].contains(&lines[location.i + 1][location.j])
    {
        r.push(Direction::South);
    }
    if !matches!(exclude, Some(Direction::West))
        && [b'S', b'-', b'J', b'7'].contains(location_char)
        && location.j > 0
        && [b'S', b'-', b'L', b'F'].contains(&lines[location.i][location.j - 1])
    {
        r.push(Direction::West);
    }
    r
}

struct BranchState {
    location: Location,
    prev_direction: Option<Direction>,
    distance: u32,
}

fn part_1() {
    let lines = read_lines("input.txt")
        .map(|s| s.into_bytes().into_boxed_slice())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let s_location = lines
        .iter()
        .enumerate()
        .find_map(|(i, s)| s.iter().position(|c| *c == b'S').map(|j| Location { i, j }))
        .expect("has S");
    let initial_connecting_pipes = find_connecting_pipe_directions(&lines, s_location, None);

    let mut left = BranchState {
        location: s_location.follow_direction(initial_connecting_pipes[0]),
        prev_direction: initial_connecting_pipes.first().copied(),
        distance: 1,
    };

    let mut right = BranchState {
        location: s_location.follow_direction(initial_connecting_pipes[1]),
        prev_direction: initial_connecting_pipes.get(1).copied(),
        distance: 1,
    };

    let mut is_left = true;

    while left.location != right.location {
        let branch = if is_left { &mut left } else { &mut right };
        let connecting_pipes = find_connecting_pipe_directions(
            &lines,
            branch.location,
            branch.prev_direction.map(Direction::opposite),
        );
        let new_direction = connecting_pipes.first().expect("has connecting pipe");
        branch.location = branch.location.follow_direction(*new_direction);
        branch.prev_direction = Some(*new_direction);
        branch.distance += 1;
        is_left = !is_left;
    }
    println!("Longest distance: {}", left.distance);
}

fn part_2() {
    let mut lines = read_lines("input.txt")
        .map(|s| s.into_bytes().into_boxed_slice())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let s_location = lines
        .iter()
        .enumerate()
        .find_map(|(i, s)| s.iter().position(|c| *c == b'S').map(|j| Location { i, j }))
        .expect("has S");
    let initial_connecting_pipes = find_connecting_pipe_directions(&lines, s_location, None);

    *s_location.in_grid_mut(&mut lines) = match initial_connecting_pipes.as_slice() {
        [Direction::North, Direction::South] | [Direction::South, Direction::North] => b'|',
        [Direction::East, Direction::West] | [Direction::West, Direction::East] => b'-',
        [Direction::East, Direction::South] | [Direction::South, Direction::East] => b'F',
        [Direction::West, Direction::South] | [Direction::South, Direction::West] => b'7',
        [Direction::North, Direction::East] | [Direction::East, Direction::North] => b'L',
        [Direction::North, Direction::West] | [Direction::West, Direction::North] => b'J',
        _ => panic!("unexpected direction pair"),
    };

    let mut path = BranchState {
        location: s_location,
        prev_direction: None,
        distance: 0,
    };

    loop {
        let connecting_pipes = find_connecting_pipe_directions(
            &lines,
            path.location,
            path.prev_direction.map(Direction::opposite),
        );
        let current_char = path.location.in_grid_mut(&mut lines);
        *current_char = match *current_char {
            b'-' => b'~',
            b'F' => b'f',
            b'L' => b'l',
            b'7' => b'>',
            b'J' => b'j',
            b'|' => b'+',
            _ => panic!("unexpected path character: {current_char}"),
        };
        if let Some(new_direction) = connecting_pipes.first() {
            path.location = path.location.follow_direction(*new_direction);
            path.prev_direction = Some(*new_direction);
            path.distance += 1;
        } else {
            break;
        }
    }

    let enclosed_sum: usize = lines
        .iter()
        .map(|line| {
            let mut sum = 0;
            let mut enclosed = false;
            let mut lhs_vertical_connector = None;
            for c in line.iter() {
                match *c {
                    b'f' | b'l' => {
                        enclosed = !enclosed;
                        lhs_vertical_connector = Some(*c);
                    }
                    b'>' | b'j' => {
                        if let Some(lhs) = lhs_vertical_connector {
                            if matches!((lhs, *c), (b'f', b'>') | (b'l', b'j')) {
                                enclosed = !enclosed;
                            }
                            lhs_vertical_connector = None;
                        } else {
                            enclosed = !enclosed;
                        }
                    }
                    b'+' => enclosed = !enclosed,
                    b'~' => (),
                    _ => {
                        if enclosed && matches!(*c, b'|' | b'-' | b'L' | b'J' | b'7' | b'F' | b'.')
                        {
                            sum += 1;
                        }
                    }
                }
            }
            sum
        })
        .sum();
    println!("Enclosed tiles: {enclosed_sum}");
}

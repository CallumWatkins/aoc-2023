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

enum Direction {
    North,
    East,
    South,
    West,
}

struct DigPlanItem {
    direction: Direction,
    distance: usize,
}

impl DigPlanItem {
    fn from_str_v1(s: &str) -> Self {
        let mut parts = s.split_ascii_whitespace();
        Self {
            direction: match parts.next().expect("has direction") {
                "U" => Direction::North,
                "R" => Direction::East,
                "D" => Direction::South,
                "L" => Direction::West,
                _ => panic!("unexpected direction character"),
            },
            distance: parts
                .next()
                .expect("has distance")
                .parse()
                .expect("has valid distance"),
        }
    }

    fn from_str_v2(s: &str) -> Self {
        let hex = s.split_ascii_whitespace().nth(2).expect("has hex part");
        Self {
            direction: match hex.as_bytes()[hex.len() - 2] {
                b'0' => Direction::East,
                b'1' => Direction::South,
                b'2' => Direction::West,
                b'3' => Direction::North,
                _ => panic!("unexpected direction character"),
            },
            distance: usize::from_str_radix(&hex[2..hex.len() - 2], 16)
                .expect("has valid hex distance"),
        }
    }
}

struct DigPlan {
    plan: Box<[DigPlanItem]>,
}

impl DigPlan {
    fn read_v1() -> Self {
        Self {
            plan: read_lines("input.txt")
                .map(|line| DigPlanItem::from_str_v1(&line))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    fn read_v2() -> Self {
        Self {
            plan: read_lines("input.txt")
                .map(|line| DigPlanItem::from_str_v2(&line))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    fn volume(&self) -> usize {
        // https://en.wikipedia.org/wiki/Shoelace_formula
        let mut shoelace_sum: isize = 0;
        let mut vertices: [(isize, isize); 2] = [(0, 0), (0, 0)];
        let mut perimeter_len: usize = 0;
        for plan_item in self.plan.iter() {
            perimeter_len += plan_item.distance;
            vertices[1] = match plan_item.direction {
                Direction::North => (
                    vertices[0]
                        .0
                        .checked_sub_unsigned(plan_item.distance)
                        .expect("path is too far north"),
                    vertices[0].1,
                ),
                Direction::East => (
                    vertices[0].0,
                    vertices[0]
                        .1
                        .checked_add_unsigned(plan_item.distance)
                        .expect("path is too far east"),
                ),
                Direction::South => (
                    vertices[0]
                        .0
                        .checked_add_unsigned(plan_item.distance)
                        .expect("path is too far south"),
                    vertices[0].1,
                ),
                Direction::West => (
                    vertices[0].0,
                    vertices[0]
                        .1
                        .checked_sub_unsigned(plan_item.distance)
                        .expect("path is too far west"),
                ),
            };
            shoelace_sum += vertices[0].0 * vertices[1].1;
            shoelace_sum -= vertices[0].1 * vertices[1].0;
            vertices[0] = vertices[1];
        }

        assert_eq!(vertices[1], (0, 0), "path should form a loop");
        (shoelace_sum.unsigned_abs() / 2) + (perimeter_len / 2) + 1
    }
}

fn part_1() {
    let plan = DigPlan::read_v1();
    let volume = plan.volume();
    println!("Lagoon volume: {volume} m^3");
}

fn part_2() {
    let plan = DigPlan::read_v2();
    let volume = plan.volume();
    println!("Lagoon volume (corrected): {volume} m^3");
}

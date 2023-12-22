use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

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

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Node {
    id: [char; 3],
}

impl Node {
    fn new(s: &str) -> Self {
        s.into()
    }
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id = ['\0'; 3];
        let mut chars = s.chars();
        for id_char in &mut id {
            let c = chars.next().ok_or(())?;
            if !c.is_ascii_uppercase() {
                return Err(());
            }
            *id_char = c;
        }
        if chars.next().is_some() {
            return Err(());
        }

        Ok(Node { id })
    }
}

fn parse_input() -> (String, HashMap<Node, (Node, Node)>) {
    let mut lines = read_lines("input.txt");
    let mut nodes: HashMap<Node, (Node, Node)> = HashMap::new();
    let directions = lines.next().expect("has first line");
    lines
        .skip(1)
        .map(|l| {
            l.split([' ', '=', '(', ',', ')'])
                .filter(|s| !s.is_empty())
                .map(std::borrow::ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .map_while(|v| {
            Some((
                v[0].parse::<Node>().ok()?,
                v[1].parse::<Node>().ok()?,
                v[2].parse::<Node>().ok()?,
            ))
        })
        .for_each(|(a, b, c)| {
            nodes.insert(a, (b, c));
        });
    (directions, nodes)
}

fn part_1() {
    let (directions, nodes) = parse_input();

    let mut current: Node = Node::new("AAA");
    let end = Node::new("ZZZ");
    let mut steps = 0;
    for direction in directions.chars().cycle() {
        steps += 1;
        current = match direction {
            'L' => nodes[&current].0,
            'R' => nodes[&current].1,
            _ => panic!("invalid direction"),
        };
        if current == end {
            break;
        }
    }
    println!("Steps to ZZZ: {steps}");
}

fn part_2() {
    let (directions, nodes) = parse_input();

    let lcm = nodes
        .keys()
        .filter(|k| k.id[2] == 'A')
        .map(|k| {
            let mut current = *k;
            let mut steps: u64 = 0;
            for direction in directions.chars().cycle() {
                steps += 1;
                current = match direction {
                    'L' => nodes[&current].0,
                    'R' => nodes[&current].1,
                    _ => panic!("invalid direction"),
                };
                if current.id[2] == 'Z' {
                    break;
                }
            }
            steps
        })
        .fold(1, num::integer::lcm);

    println!("Steps until nodes end with Z: {lcm}");
}

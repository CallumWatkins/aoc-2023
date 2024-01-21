use lazy_static::lazy_static;
use regex::Regex;
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

#[derive(Copy, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

impl From<u8> for Category {
    fn from(value: u8) -> Self {
        match value {
            b'x' => Self::X,
            b'm' => Self::M,
            b'a' => Self::A,
            b's' => Self::S,
            _ => panic!("unexpected category character"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct WorkflowName(u16);

impl FromStr for WorkflowName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(
            s.len() <= 3,
            "cannot encode name longer than 3 chars as u16"
        );
        let mut result = 0u16;
        for b in s.bytes() {
            result *= 26;
            result += u16::from(b - b'a');
        }
        Ok(Self(result))
    }
}

enum Rule {
    Accept,
    Reject,
    Defer(WorkflowName),
    LtAccept(Category, u16),
    LtReject(Category, u16),
    LtDefer(Category, u16, WorkflowName),
    GtAccept(Category, u16),
    GtReject(Category, u16),
    GtDefer(Category, u16, WorkflowName),
}

struct Workflow {
    name: WorkflowName,
    rules: Box<[Rule]>,
}

lazy_static! {
    static ref RULE_REGEX: Regex =
        Regex::new(r"(?m)([a-z])(<|>)(\d+):([a-z]+|A|R)|[a-z]+|A|R").unwrap();
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = s.split_once('{').expect("valid workflow");
        Ok(Self {
            name: name.parse().unwrap(),
            rules: RULE_REGEX
                .captures_iter(&rest[0..rest.len() - 1])
                .map(|captures| {
                    if captures.get(1).is_none() {
                        match &captures[0] {
                            "A" => Rule::Accept,
                            "R" => Rule::Reject,
                            dst => Rule::Defer(dst.parse().unwrap()),
                        }
                    } else {
                        let cat: Category = captures[1].as_bytes()[0].into();
                        let val: u16 = captures[3].parse().expect("rule constant is valid number");
                        let dst = &captures[4];
                        match (captures[2].as_bytes()[0], dst) {
                            (b'<', "A") => Rule::LtAccept(cat, val),
                            (b'<', "R") => Rule::LtReject(cat, val),
                            (b'>', "A") => Rule::GtAccept(cat, val),
                            (b'>', "R") => Rule::GtReject(cat, val),
                            (b'<', _) => Rule::LtDefer(cat, val, dst.parse().unwrap()),
                            (b'>', _) => Rule::GtDefer(cat, val, dst.parse().unwrap()),
                            _ => panic!("unexpected comparator"),
                        }
                    }
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        })
    }
}

#[derive(Copy, Clone)]
struct MachinePart {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl MachinePart {
    fn cat(self, cat: Category) -> u16 {
        match cat {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn set_cat(&mut self, cat: Category, val: u16) {
        match cat {
            Category::X => self.x = val,
            Category::M => self.m = val,
            Category::A => self.a = val,
            Category::S => self.s = val,
        }
    }
}

impl FromStr for MachinePart {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut part = Self {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };

        s[1..s.len() - 1].split(',').for_each(|v| {
            part.set_cat(
                v.as_bytes()[0].into(),
                v[2..].parse().expect("valid u16 rating"),
            );
        });

        Ok(part)
    }
}

struct System {
    workflows: Box<[Workflow]>,
    machine_parts: Box<[MachinePart]>,
}

impl System {
    fn read() -> Self {
        let mut lines = read_lines("input.txt");
        Self {
            workflows: lines
                .by_ref()
                .take_while(|line| !line.is_empty())
                .map(|line| line.parse().unwrap())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            machine_parts: lines
                .map(|line| line.parse().unwrap())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    fn rating_sum(&self) -> usize {
        let workflow_map: HashMap<WorkflowName, &Workflow> =
            self.workflows.iter().map(|w| (w.name, w)).collect();

        self.machine_parts
            .iter()
            .filter(|&&part| {
                let mut workflow = *workflow_map
                    .get(&WorkflowName::from_str("in").unwrap())
                    .expect("has 'in' workflow");
                loop {
                    let mut next_workflow = None;
                    for rule in workflow.rules.iter() {
                        if let Some(dst) = match rule {
                            Rule::Accept => return true,
                            Rule::Reject => return false,
                            Rule::Defer(dst) => Some(dst),
                            Rule::LtAccept(cat, val) if part.cat(*cat) < *val => return true,
                            Rule::LtReject(cat, val) if part.cat(*cat) < *val => return false,
                            Rule::GtAccept(cat, val) if part.cat(*cat) > *val => return true,
                            Rule::GtReject(cat, val) if part.cat(*cat) > *val => return false,
                            Rule::LtDefer(cat, val, dst) if part.cat(*cat) < *val => Some(dst),
                            Rule::GtDefer(cat, val, dst) if part.cat(*cat) > *val => Some(dst),
                            _ => None,
                        } {
                            next_workflow = Some(*workflow_map.get(dst).expect("has destination"));
                            break;
                        }
                    }
                    if let Some(next_w) = next_workflow {
                        workflow = next_w;
                    } else {
                        panic!("no rules matched");
                    }
                }
            })
            .map(|w| w.x as usize + w.m as usize + w.a as usize + w.s as usize)
            .sum()
    }

    fn permutations(&self) -> usize {
        fn range_permutations(lower: MachinePart, upper: MachinePart) -> usize {
            let mut perms = 1usize;
            perms *= (upper.x - lower.x + 1) as usize;
            perms *= (upper.m - lower.m + 1) as usize;
            perms *= (upper.a - lower.a + 1) as usize;
            perms *= (upper.s - lower.s + 1) as usize;
            perms
        }

        let workflow_map: HashMap<WorkflowName, &Workflow> =
            self.workflows.iter().map(|w| (w.name, w)).collect();
        let mut sum = 0usize;
        let mut stack: Vec<(MachinePart, MachinePart, &Workflow)> = vec![(
            MachinePart {
                x: 1,
                m: 1,
                a: 1,
                s: 1,
            },
            MachinePart {
                x: 4000,
                m: 4000,
                a: 4000,
                s: 4000,
            },
            *workflow_map
                .get(&WorkflowName::from_str("in").unwrap())
                .expect("has 'in' workflow"),
        )];

        while let Some((mut lower, mut upper, workflow)) = stack.pop() {
            for rule in workflow.rules.iter() {
                match rule {
                    Rule::Accept => sum += range_permutations(lower, upper),
                    Rule::Reject => break,
                    Rule::Defer(dst) => {
                        stack.push((lower, upper, *workflow_map.get(dst).unwrap()));
                    }
                    Rule::LtAccept(cat, val) if lower.cat(*cat) < *val => {
                        let mut new_upper = upper;
                        new_upper.set_cat(*cat, *val - 1);
                        sum += range_permutations(lower, new_upper);
                    }
                    Rule::GtAccept(cat, val) if upper.cat(*cat) > *val => {
                        let mut new_lower = lower;
                        new_lower.set_cat(*cat, *val + 1);
                        sum += range_permutations(new_lower, upper);
                    }
                    Rule::LtDefer(cat, val, dst) if lower.cat(*cat) < *val => {
                        let mut new_upper = upper;
                        new_upper.set_cat(*cat, *val - 1);
                        stack.push((lower, new_upper, *workflow_map.get(dst).unwrap()));
                    }
                    Rule::GtDefer(cat, val, dst) if upper.cat(*cat) > *val => {
                        let mut new_lower = lower;
                        new_lower.set_cat(*cat, *val + 1);
                        stack.push((new_lower, upper, *workflow_map.get(dst).unwrap()));
                    }
                    _ => (),
                }
                match rule {
                    Rule::Accept | Rule::Reject | Rule::Defer(_) => (),
                    Rule::LtAccept(cat, val)
                    | Rule::LtReject(cat, val)
                    | Rule::LtDefer(cat, val, _)
                        if upper.cat(*cat) >= *val =>
                    {
                        lower.set_cat(*cat, *val);
                    }
                    Rule::GtAccept(cat, val)
                    | Rule::GtReject(cat, val)
                    | Rule::GtDefer(cat, val, _)
                        if lower.cat(*cat) <= *val =>
                    {
                        upper.set_cat(*cat, *val);
                    }
                    _ => break,
                }
            }
        }

        sum
    }
}

fn part_1() {
    let system = System::read();
    let sum = system.rating_sum();
    println!("Accepted ratings sum: {sum}");
}

fn part_2() {
    let system = System::read();
    let perms = system.permutations();
    println!("Acceptable ratings permutations sum: {perms}");
}

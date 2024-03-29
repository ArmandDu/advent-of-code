use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;

struct Day19;

#[derive(Debug)]
struct Workshop {
    workflows: HashMap<String, Vec<Operation>>,
    parts: Vec<HashMap<char, usize>>,
}

#[derive(Debug)]
enum Operation {
    GT(char, usize, String),
    LT(char, usize, String),
    Move(String),
}

impl Operation {
    fn validate(&self, parts: &HashMap<char, usize>) -> Option<&str> {
        match self {
            Operation::GT(part, count, dest) => {
                (parts.get(part) > Some(count)).then_some(dest.as_str())
            }
            Operation::LT(part, count, dest) => (parts.get(part) < Some(count)).then_some(dest),
            Operation::Move(dest) => Some(dest),
        }
    }
}

impl FromStr for Operation {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((op, redir)) = s.split_once(':') {
            let (name_cond, count) = op
                .split_inclusive(['<', '>'])
                .collect_tuple()
                .ok_or(SolutionError::ParseError)?;
            let count = count.parse().map_err(|_| SolutionError::ParseError)?;

            let (name, cond) = name_cond.split_at(name_cond.len() - 1);
            let name = name.chars().next().ok_or(SolutionError::ParseError)?;

            Ok(match cond {
                ">" => Operation::GT(name.to_owned(), count, redir.to_owned()),
                "<" => Operation::LT(name.to_owned(), count, redir.to_owned()),
                _ => return Err(SolutionError::ParseError),
            })
        } else {
            Ok(Operation::Move(s.to_owned()))
        }
    }
}

impl FromStr for Workshop {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.replace('\r', "");

        let (workflows, parts) = input.split_once("\n\n").ok_or(SolutionError::ParseError)?;

        let workflows = workflows
            .lines()
            .map(|line| {
                let (name, checks) = line.split_once('{').ok_or(SolutionError::ParseError)?;

                let operations = checks.replace(['{', '}'], "");

                let operations = operations
                    .split(',')
                    .map(Operation::from_str)
                    .collect::<Result<Vec<_>, _>>()?;

                Ok((name.to_owned(), operations))
            })
            .collect::<Result<HashMap<_, _>, SolutionError>>()?;

        let parts = parts
            .lines()
            .map(|line| {
                let line = line.replace(['{', '}'], "");

                line.split(',')
                    .map(|part_count| {
                        let (part, count) = part_count.split_once('=')?;
                        let part = part.chars().next()?;

                        Some((part, count.parse::<usize>().ok()?))
                    })
                    .collect::<Option<HashMap<_, _>>>()
                    .ok_or(SolutionError::ParseError)
            })
            .collect::<Result<Vec<_>, SolutionError>>()?;

        Ok(Workshop { workflows, parts })
    }
}

impl Workshop {
    fn accepted_parts(&self) -> impl Iterator<Item = &HashMap<char, usize>> {
        self.parts.iter().filter(|parts| {
            let mut current = "in";

            while let Some(operations) = self.workflows.get(current) {
                if let Some(next) = operations.iter().find_map(|op| op.validate(parts)) {
                    match next {
                        "R" => return false,
                        "A" => return true,
                        dest => current = dest,
                    }
                }
            }

            false
        })
    }

    fn accepted_ranges(
        &self,
        start: HashMap<char, (usize, usize)>,
    ) -> Vec<HashMap<char, (usize, usize)>> {
        let mut queue = VecDeque::new();
        let mut results = Vec::new();

        queue.push_back((start, "in"));

        while let Some((mut current_parts, name)) = queue.pop_front() {
            if let Some(operations) = self.workflows.get(name) {
                for op in operations {
                    let (next_part, next) = match op {
                        Operation::GT(part, count, dest) => {
                            let mut valid_parts = current_parts.to_owned();

                            valid_parts.get_mut(part).unwrap().0 = count + 1;
                            current_parts.get_mut(part).unwrap().1 = *count;

                            (valid_parts, dest)
                        }
                        Operation::LT(part, count, dest) => {
                            let mut valid_parts = current_parts.to_owned();

                            valid_parts.get_mut(part).unwrap().1 = count - 1;
                            current_parts.get_mut(part).unwrap().0 = *count;

                            (valid_parts, dest)
                        }
                        Operation::Move(dest) => (current_parts.to_owned(), dest),
                    };

                    match next.as_str() {
                        "A" => results.push(next_part),
                        "R" => {}
                        _ => queue.push_back((next_part, next)),
                    }
                }
            }
        }
        results
    }
}

impl Solution for Day19 {
    const TITLE: &'static str = "Aplenty";
    const DAY: u8 = 19;
    type Input = Workshop;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Workshop::from_str(input)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        input
            .accepted_parts()
            .map(|parts| parts.values().sum::<usize>())
            .sum1()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let start: HashMap<_, _> = "xmas".chars().map(|c| (c, (1, 4000))).collect();

        input
            .accepted_ranges(start)
            .iter()
            .map(|parts| {
                parts
                    .values()
                    .map(|(start, end)| end - start + 1)
                    .product::<usize>()
            })
            .sum1()
    }
}

aoc::run!(Day19);

aoc::example! {
    [Day19]
    sample: "px{a<2006:qkq,m>2090:A,rfg}\r\npv{a>1716:R,A}\r\nlnx{m>1548:A,A}\r\nrfg{s<537:gd,x>2440:R,A}\r\nqs{s>3448:A,lnx}\r\nqkq{x<1416:A,crn}\r\ncrn{x>2662:A,R}\r\nin{s<1351:px,qqz}\r\nqqz{s>2770:qs,m<1801:hdj,R}\r\ngd{a>3333:R,R}\r\nhdj{m>838:A,pv}\r\n\r\n{x=787,m=2655,a=1222,s=2876}\r\n{x=1679,m=44,a=2067,s=496}\r\n{x=2036,m=264,a=79,s=2244}\r\n{x=2461,m=1339,a=466,s=291}\r\n{x=2127,m=1623,a=2188,s=1013}\r\n"
        => Some(19114)
        => Some(167_409_079_868_000)
}

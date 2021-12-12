use crate::day10::State::{Complete, Corrupted, Incomplete};
use itertools::Itertools;
use year2021::Solution;

pub struct Day10;

pub enum State {
    Complete,
    Corrupted(Vec<char>),
    Incomplete(Vec<char>),
}

impl Solution<usize, usize> for Day10 {
    const DAY: u32 = 10;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Syntax Scoring";
    type Input = Vec<State>;

    fn part1(input: &Self::Input) -> Option<usize> {
        let all_corrupted = input
            .iter()
            .filter_map(|result| match result {
                Corrupted(stack) => stack.first(),
                _ => None,
            })
            .collect::<Vec<_>>();

        Some(
            all_corrupted
                .iter()
                .map(|&c| match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!(),
                })
                .sum::<usize>(),
        )
    }

    fn part2(input: &Self::Input) -> Option<usize> {
        let all_incomplete = input
            .iter()
            .filter_map(|result| match result {
                Incomplete(stack) => Some(stack),
                _ => None,
            })
            .collect::<Vec<_>>();

        let score: Vec<_> = all_incomplete
            .iter()
            .map(|stack| {
                stack.iter().rev().fold(0, |acc, c| {
                    (acc * 5)
                        + match c {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => panic!(),
                        }
                })
            })
            .sorted()
            .collect();

        score.get(score.len() / 2).cloned()
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        Ok(input
            .lines()
            .map(|s| String::from(s.trim()))
            .map(|line| {
                let mut stack = vec![];
                let mut corrupted = vec![];

                for c in line.chars() {
                    match c {
                        '(' | '[' | '{' | '<' => stack.push(c),
                        ')' if stack.pop() == Some('(') => {}
                        ']' if stack.pop() == Some('[') => {}
                        '}' if stack.pop() == Some('{') => {}
                        '>' if stack.pop() == Some('<') => {}
                        c => {
                            corrupted.push(c);
                        }
                    }
                }

                match (corrupted.len(), stack.len()) {
                    (n, _) if n > 0 => Corrupted(corrupted),
                    (_, n) if n > 0 => Incomplete(stack),
                    _ => Complete,
                }
            })
            .collect())
    }
}

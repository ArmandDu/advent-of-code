use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
struct Oasis(Vec<Vec<i32>>);

impl FromStr for Oasis {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| line.split_whitespace().map(|hist| hist.parse()).collect())
            .collect::<Result<Vec<_>, _>>()
            .map(Oasis::new)
            .map_err(|_| SolutionError::ParseError)
    }
}

impl Oasis {
    pub fn new(history: Vec<Vec<i32>>) -> Self {
        Self(history.iter().map(|hist| Self::predict(hist)).collect())
    }
    fn predict(history: &[i32]) -> Vec<i32> {
        let mut last = vec![history.last().cloned().unwrap_or_default()];
        let mut first = vec![history.first().cloned().unwrap_or_default()];

        let mut diff = history.to_owned();

        loop {
            diff = diff
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect_vec();

            first.push(diff.first().cloned().unwrap_or_default());
            last.push(diff.last().cloned().unwrap_or_default());

            if diff.iter().all_equal() {
                break;
            }
        }

        let first = first
            .iter()
            .cloned()
            .rev()
            .reduce(|sum, diff| diff - sum)
            .unwrap_or_default();

        let last = last.iter().sum();

        [vec![first], history.to_vec(), vec![last]].concat()
    }
}

struct Day09;

impl Solution for Day09 {
    const TITLE: &'static str = "Mirage Maintenance";
    const DAY: u8 = 9;
    type Input = Oasis;
    type P1 = i32;
    type P2 = i32;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Oasis::from_str(input)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        input.0.iter().map(|hist| hist.last()).sum()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        input.0.iter().map(|hist| hist.first()).sum()
    }
}

aoc::run!(Day09);

aoc::example! {
    [Day09]
    example: "0 3 6 9 12 15\r\n1 3 6 10 15 21\r\n10 13 16 21 30 45\r\n"
        => Some(114)
        => Some(2)
}

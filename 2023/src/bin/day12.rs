use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
struct SpringRecord(Vec<(String, Vec<usize>)>);

impl SpringRecord {
    fn counts(record: &str) -> Vec<usize> {
        record
            .chars()
            .group_by(|&c| c == '#')
            .into_iter()
            .filter_map(|(in_group, group)| in_group.then_some(group.count()))
            .collect_vec()
    }

    fn is_valid(record: &str, validator: &[usize]) -> bool {
        Self::counts(record) == validator
    }

    fn next_index(record: &[char]) -> usize {
        record
            .iter()
            .find_position(|&c| c == &'?')
            .map(|(p, _)| p)
            .unwrap_or(record.len())
    }

    fn solve(record: &str, validator: &[usize]) -> Option<Vec<String>> {
        let mut history = Vec::new();
        let mut queue = VecDeque::new();

        if Self::is_valid(record, validator) {
            return Some(vec![record.to_owned()]);
        }

        let record = record.chars().collect_vec();
        let start = Self::next_index(&record);

        queue.push_back((start, record));

        while let Some((index, current)) = queue.pop_front() {
            let current_string = current.iter().collect::<String>();

            if Self::is_valid(&current_string, validator) {
                history.push(current_string.replace('?', "."));
                continue;
            }

            if index >= current.len() {
                continue;
            }

            for c in ['.', '#'] {
                let mut copy = current.to_owned();
                let elem = copy.get_mut(index)?;

                *elem = c;

                let next = Self::next_index(&copy);

                // TODO find out how to prune invalid branches
                queue.push_back((next, copy))
            }
        }

        Some(history)
    }
}

impl FromStr for SpringRecord {
    type Err = SolutionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input
            .lines()
            .map(|line| {
                let (record, cont) = line.split_once(' ').ok_or(SolutionError::ParseError)?;
                let cont = cont
                    .split(',')
                    .map(|num| num.parse())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| SolutionError::ParseError)?;

                Ok((record.to_owned(), cont))
            })
            .collect::<Result<_, _>>()
            .map(SpringRecord)
    }
}

struct Day12;

impl Solution for Day12 {
    const TITLE: &'static str = "Hot Springs";
    const DAY: u8 = 12;
    type Input = SpringRecord;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        SpringRecord::from_str(input)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        input
            .0
            .iter()
            .filter_map(|(record, val)| SpringRecord::solve(record, val))
            .map(|hist| hist.len())
            .sum1()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let _ = input.0.iter().map(|(record, val)| {
            (
                std::iter::repeat(record).take(5).join(""),
                std::iter::repeat(val.to_owned())
                    .take(5)
                    .flatten()
                    .collect_vec(),
            )
        });

        None
    }
}

aoc::run!(Day12);

aoc::example! {
    [Day12]
    small: "???.### 1,1,3\r\n.??..??...?##. 1,1,3\r\n?#?#?#?#?#?#?#? 1,3,1,6\r\n????.#...#... 4,1,1\r\n????.######..#####. 1,6,5\r\n?###???????? 3,2,1\r\n"
        => Some(21)
        => None
}

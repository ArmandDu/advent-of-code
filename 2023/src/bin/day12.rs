use aoc::solution::SolutionError;
use aoc::Solution;
use aoc_utils::pointer::Pointer;
use itertools::Itertools;
use memoize::memoize;
use rayon::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
struct SpringRecord(Vec<(String, Vec<usize>)>);

#[memoize]
fn solve(record: Pointer<char>, validator: Pointer<usize>) -> usize {
    //all other checks passed. Now checking that the remaining is also empty.
    if validator.is_empty() {
        return record.iter().all(|&c| c != '#').into();
    }

    //still '#' to find but not enough space left
    if record.iter().count() < validator.iter().sum::<usize>() + validator.iter().count() - 1 {
        return 0;
    }

    //skip until next non '.'
    if let Some('.') = record.first() {
        return solve(record.move_cursor(1), validator);
    }

    //record.first is '?' or '#'

    let n_valid = {
        // check if token is valid. It's valid if validator.count first tokens are '?' or '#' and count+1 is end, '.' or '?'
        let count = validator.first().copied().unwrap();
        let is_valid = record.iter().take(count).all(|&c| c != '.');
        let next_is_valid = record.get(count).map(|&c| c != '#').unwrap_or(true);

        (is_valid && next_is_valid)
            .then(|| {
                solve(
                    record.to_owned().move_cursor(count + 1),
                    validator.to_owned().move_cursor(1),
                )
            })
            .unwrap_or_default()
    } + {
        // if current token is '?' try marking it as '.' by skipping the token.
        if let Some('?') = record.first() {
            solve(record.move_cursor(1), validator)
        } else {
            0
        }
    };

    n_valid
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
            .map(|(record, val)| {
                solve(
                    Pointer::from_iter(record.chars()),
                    Pointer::new(val.to_vec()),
                )
            })
            .sum1()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        Some(
            input
                .0
                .par_iter()
                .map(|(record, val)| {
                    (
                        Pointer::from_iter((0..5).map(|_| record).join("?").chars()),
                        Pointer::from_iter((0..5).flat_map(|_| val).copied()),
                    )
                })
                .map(|(record, val)| solve(record, val))
                .sum(),
        )
    }
}

aoc::run!(Day12);

aoc::example! {
    [Day12]
    small: "???.### 1,1,3\r\n.??..??...?##. 1,1,3\r\n?#?#?#?#?#?#?#? 1,3,1,6\r\n????.#...#... 4,1,1\r\n????.######..#####. 1,6,5\r\n?###???????? 3,2,1\r\n"
        => Some(21)
        => Some(525152)
}

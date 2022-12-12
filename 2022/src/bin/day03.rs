use aoc::Solution;
use shared::lines_to_owned;
use std::collections::HashSet;

struct Day03;

impl Day03 {
    fn priority(c: char) -> usize {
        if c.is_lowercase() {
            ((c as u8) - b'a' + 1) as usize
        } else {
            ((c as u8) - b'A' + 27) as usize
        }
    }
}

impl Solution for Day03 {
    const TITLE: &'static str = "Rucksack Reorganization";
    const DAY: u8 = 3;
    type Input = Vec<String>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Ok(lines_to_owned(input))
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        Some(
            input
                .iter()
                .map(|line| {
                    let (left, right) = line.split_at(&line.len() / 2);

                    (
                        left.chars().map(Day03::priority).collect::<HashSet<_>>(),
                        right.chars().map(Day03::priority).collect::<HashSet<_>>(),
                    )
                })
                .map(|(left, right)| {
                    left.iter()
                        .filter(|&value| right.contains(value))
                        .sum::<usize>()
                })
                .sum(),
        )
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        Some(
            input
                .chunks_exact(3)
                .map(|chunk| {
                    if let [first, second, third] = chunk {
                        let first: HashSet<_> = first.chars().map(Day03::priority).collect();
                        let second: HashSet<_> = second.chars().map(Day03::priority).collect();
                        let third: HashSet<_> = third.chars().map(Day03::priority).collect();

                        first
                            .iter()
                            .filter(|&value| second.contains(value) && third.contains(value))
                            .sum()
                    } else {
                        0
                    }
                })
                .sum(),
        )
    }
}

fn main() {
    aoc::solution!(Day03)
}
#[cfg(test)]
mod tests {
    use crate::Day03 as day_03;
    use crate::*;

    aoc::test_common!(day_03);

    aoc::test! {
        day_03:
        - "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n"
            => Some(157)
            => Some(70)
    }
}

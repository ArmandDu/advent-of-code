use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;
use std::str::FromStr;

struct Day04;

#[derive(Debug)]
struct ScratchCard(Vec<u32>, Vec<u32>);

impl ScratchCard {
    pub fn winning_numbers(&self) -> Vec<&u32> {
        self.1
            .iter()
            .filter(|number| self.0.contains(number))
            .collect_vec()
    }
}

impl FromStr for ScratchCard {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, numbers) = s.split_once(':').ok_or(SolutionError::ParseError)?;
        let (winning, game) = numbers.split_once('|').ok_or(SolutionError::ParseError)?;

        Ok(ScratchCard(
            winning
                .trim()
                .split(' ')
                .filter_map(|num| num.parse::<u32>().ok())
                .collect(),
            game.trim()
                .split(' ')
                .filter_map(|num| num.parse::<u32>().ok())
                .collect(),
        ))
    }
}

impl Solution for Day04 {
    const TITLE: &'static str = "Scratchcards";
    const DAY: u8 = 4;
    type Input = Vec<ScratchCard>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        input.lines().map(ScratchCard::from_str).collect()
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        input
            .iter()
            .map(ScratchCard::winning_numbers)
            .filter_map(|game| match game.len() {
                0 => None,
                x => Some(x - 1),
            })
            .map(|game| 2_usize.pow(game as u32))
            .sum1()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let initial = vec![1; input.len()];

        input
            .iter()
            .map(|card| card.winning_numbers().len())
            .enumerate()
            .map(|(i, count)| i + 1..(i + 1) + count)
            .enumerate()
            .fold(initial, |mut v, (i, range)| {
                range.for_each(|j| v[j] += v[i]);

                v
            })
            .iter()
            .sum1()
    }
}

fn main() {
    aoc::solution!(Day04)
}
#[cfg(test)]
mod tests {
    use crate::Day04 as day_04;
    use crate::*;

    aoc::test_common!(day_04);

    aoc::test! {
        day_04:
        [example]
        - "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\r\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\r\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\r\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\r\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\r\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\r\n"
            => Some(13)
            => Some(30);

    }
}

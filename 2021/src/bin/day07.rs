use aoc::Solution;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day07;

impl Solution<usize, usize> for Day07 {
    const DAY: u32 = 7;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "The Treachery of Whales";
    type Input = Vec<i32>;

    fn part1(input: &Self::Input) -> Option<usize> {
        let median_index = (input.len() + 1) / 2;
        let median_pos = input.get(median_index).unwrap();

        let fuel_cost = input
            .iter()
            .map(|pos| (pos - median_pos).abs())
            .sum::<i32>();

        Some(fuel_cost as usize)
    }

    fn part2(input: &Self::Input) -> Option<usize> {
        let crabs: HashMap<_, _> = input.into_iter().fold(HashMap::new(), |mut pool, &pos| {
            *pool.entry(pos).or_insert(0) += 1;
            pool
        });

        let (_pos, fuel_cost) = (*crabs.keys().min().unwrap()..*crabs.keys().max().unwrap())
            .map(|target_pos| {
                (
                    target_pos,
                    crabs
                        .iter()
                        .map(|(crab_pos, crab_count)| {
                            let dist = (crab_pos - target_pos).abs();

                            ((dist * (dist + 1)) / 2) * crab_count
                        })
                        .sum::<i32>(),
                )
            })
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        Some(fuel_cost as usize)
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        Ok(input
            .trim()
            .split(",")
            .map(|input| input.parse::<i32>().unwrap())
            .sorted()
            .collect::<Vec<_>>())
    }
}

fn main() {
    Day07::run(include_str!("../../data/day07_input"));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn day07() {
        Day07::test(INPUT, Some(37), Some(168));
    }
}

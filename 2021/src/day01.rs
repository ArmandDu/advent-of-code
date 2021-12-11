use itertools::Itertools;
use year2021::Solution;

pub struct Day01;

impl Solution<usize, usize> for Day01 {
    const DAY: u32 = 1;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Sonar Sweep";
    type Input = Vec<i32>;

    fn part1(input: &Self::Input) -> Option<usize> {
        Some(input.iter().tuple_windows().filter(|(a, b)| a < b).count())
    }

    fn part2(input: &Self::Input) -> Option<usize> {
        Some(
            input
                .iter()
                .tuple_windows()
                .filter(|(a, _, _, d)| a < d)
                .count(),
        )
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        Ok(input
            .trim()
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            .collect())
    }
}

use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;
use std::str::FromStr;

struct Day06;

#[derive(Debug)]
struct Puzzle(Vec<(usize, usize)>);

impl Puzzle {
    fn get_bounds(time: usize, distance: usize) -> Option<usize> {
        let min = (1..time).find(|ms| ms * (time - ms) > distance)?;
        let max = (1..time).rev().find(|ms| ms * (time - ms) > distance)?;

        Some(max - min + 1)
    }
}

impl FromStr for Puzzle {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (time, distance) = s.lines().collect_tuple().ok_or(SolutionError::ParseError)?;
        let time = time
            .replace("Time:", "")
            .split_whitespace()
            .map(|num| num.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| SolutionError::ParseError)?;
        let distance = distance
            .replace("Distance:", "")
            .split_whitespace()
            .map(|num| num.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| SolutionError::ParseError)?;

        Ok(Puzzle(time.into_iter().zip(distance).collect()))
    }
}

impl Solution for Day06 {
    const TITLE: &'static str = "Wait For It";
    const DAY: u8 = 6;
    type Input = Puzzle;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Puzzle::from_str(input)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        input
            .0
            .iter()
            .map(|(time, distance)| Puzzle::get_bounds(*time, *distance))
            .product()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let (time, distance): (Vec<_>, Vec<_>) = input.0.iter().cloned().unzip();
        let time = time.iter().join("").parse::<usize>().ok()?;
        let distance = distance.iter().join("").parse::<usize>().ok()?;

        Puzzle::get_bounds(time, distance)
    }
}

aoc::run!(Day06);

aoc::example! {
    [Day06]
    example: "Time:      7  15   30\r\nDistance:  9  40  200\r\n"
        => Some(288)
        => Some(71503)
}

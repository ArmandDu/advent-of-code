use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;

struct Day18;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point3(i8, i8, i8);

impl FromStr for Point3 {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .split(',')
            .map(i8::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| SolutionError::ParseError)?;

        match v.iter().collect_tuple() {
            Some((x, y, z)) => Ok(Self(*x, *y, *z)),
            _ => Err(SolutionError::ParseError),
        }
    }
}

impl From<(i8, i8, i8)> for Point3 {
    fn from((x, y, z): (i8, i8, i8)) -> Self {
        Self(x, y, z)
    }
}

impl Point3 {
    fn neighbors(&self) -> [Point3; 6] {
        let &Point3(x, y, z) = self;

        [
            Point3(x + 1, y, z),
            Point3(x - 1, y, z),
            Point3(x, y + 1, z),
            Point3(x, y - 1, z),
            Point3(x, y, z + 1),
            Point3(x, y, z - 1),
        ]
    }

    fn within(&self, min: &Self, max: &Self) -> bool {
        min.0 <= self.0
            && self.0 <= max.0
            && min.1 <= self.1
            && self.1 <= max.1
            && min.2 <= self.2
            && self.2 <= max.2
    }
}

impl Solution for Day18 {
    const TITLE: &'static str = "Boiling Boulders";
    const DAY: u8 = 18;
    type Input = HashSet<Point3>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        input.lines().map(Point3::from_str).collect()
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        Some(
            input
                .iter()
                .map(|point| {
                    point
                        .neighbors()
                        .into_iter()
                        .filter(|neighbor| !input.contains(neighbor))
                        .count()
                })
                .sum(),
        )
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let x_range = input.iter().map(|p| p.0).minmax().into_option()?;
        let y_range = input.iter().map(|p| p.1).minmax().into_option()?;
        let z_range = input.iter().map(|p| p.2).minmax().into_option()?;

        let min_range = Point3(x_range.0 - 1, y_range.0 - 1, z_range.0 - 1);
        let max_range = Point3(x_range.1 + 1, y_range.1 + 1, z_range.1 + 1);

        let mut cache = HashSet::new();
        let mut queue = VecDeque::new();
        let mut count = 0;

        queue.push_back(min_range.clone());

        while let Some(air) = queue.pop_front() {
            if !cache.contains(&air) {
                cache.insert(air.clone());

                for neighbor in air.neighbors() {
                    if neighbor.within(&min_range, &max_range) {
                        if input.contains(&neighbor) {
                            count += 1;
                        } else {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }

        Some(count)
    }
}

fn main() {
    aoc::solution!(Day18)
}
#[cfg(test)]
mod tests {
    use crate::Day18 as day_18;
    use crate::*;

    aoc::test_common!(day_18);

    aoc::test! {
        day_18:
        - "2,2,2\n1,2,2\n3,2,2\n2,1,2\n2,3,2\n2,2,1\n2,2,3\n2,2,4\n2,2,6\n1,2,5\n3,2,5\n2,1,5\n2,3,5\n"
            => Some(64)
            => Some(58)
    }
}

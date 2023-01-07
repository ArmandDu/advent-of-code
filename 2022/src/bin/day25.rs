use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

struct Day25;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Snafu(isize);

impl FromStr for Snafu {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut decimal = 0;

        for (index, char) in s.chars().rev().enumerate() {
            let numerator = match char {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => return Err(SolutionError::ParseError),
            };

            decimal += numerator * 5_isize.pow(index as u32)
        }

        Ok(Self(decimal))
    }
}

impl From<isize> for Snafu {
    fn from(value: isize) -> Self {
        Self(value)
    }
}

impl Snafu {
    fn as_decimal(&self) -> isize {
        self.0
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut snafu = vec![];
        let mut decimal = self.0;

        while decimal > 0 {
            snafu.push(match decimal % 5 {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => {
                    decimal += 2;
                    '='
                }
                4 => {
                    decimal += 1;
                    '-'
                }
                _ => unreachable!(),
            });

            decimal /= 5;
        }

        write!(f, "{}", snafu.iter().rev().join(""))
    }
}

impl Solution for Day25 {
    const TITLE: &'static str = "Full of Hot Air";
    const DAY: u8 = 25;
    type Input = Vec<Snafu>;
    type P1 = Snafu;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        input.lines().map(Snafu::from_str).collect()
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        Some(Snafu::from(
            input.iter().map(|n| n.as_decimal()).sum::<isize>(),
        ))
    }

    fn part2(_input: &Self::Input) -> Option<Self::P2> {
        None
    }
}

fn main() {
    aoc::solution!(Day25)
}
#[cfg(test)]
mod tests {
    use crate::Day25 as day_25;
    use crate::*;

    aoc::test_common!(day_25);

    aoc::test! {
        day_25:
        - "1=-0-2\n12111\n2=0=\n21\n2=01\n111\n20012\n112\n1=-1=\n1-12\n12\n1=\n122\n"
            => "2=-1=0".parse().ok()
            => None
    }
}

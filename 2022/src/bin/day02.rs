use aoc::Solution;

struct Day02;

enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from(input: &str) -> Self {
        match input {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }

    fn battle(&self, other: &Self) -> usize {
        match (self, other) {
            (Self::Rock, Self::Paper) => 0,
            (Self::Rock, Self::Rock) => 3,
            (Self::Rock, Self::Scissors) => 6,
            (Self::Paper, Self::Paper) => 3,
            (Self::Paper, Self::Rock) => 6,
            (Self::Paper, Self::Scissors) => 0,
            (Self::Scissors, Self::Paper) => 6,
            (Self::Scissors, Self::Rock) => 0,
            (Self::Scissors, Self::Scissors) => 3,
        }
    }

    fn points(&self) -> usize {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

impl Solution for Day02 {
    const TITLE: &'static str = "Rock Paper Scissors";
    const DAY: u8 = 2;
    type Input = Vec<(String, String)>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Ok(input
            .lines()
            .filter_map(|line| {
                line.split_once(' ')
                    .map(|(a, b)| (a.to_owned(), b.to_owned()))
            })
            .collect())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        Some(
            input
                .iter()
                .map(|(elf, me)| {
                    let elf = RPS::from(&elf[..]);
                    let me = RPS::from(&me[..]);

                    me.battle(&elf) + me.points()
                })
                .sum(),
        )
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        Some(
            input
                .iter()
                .map(|(elf, me)| {
                    let elf = RPS::from(&elf[..]);
                    let me = match (&elf, &me[..]) {
                        (RPS::Rock, "X") => RPS::Scissors,
                        (RPS::Rock, "Y") => RPS::Rock,
                        (RPS::Rock, "Z") => RPS::Paper,
                        (RPS::Paper, "X") => RPS::Rock,
                        (RPS::Paper, "Y") => RPS::Paper,
                        (RPS::Paper, "Z") => RPS::Scissors,
                        (RPS::Scissors, "X") => RPS::Paper,
                        (RPS::Scissors, "Y") => RPS::Scissors,
                        (RPS::Scissors, "Z") => RPS::Rock,
                        _ => unreachable!(),
                    };

                    me.battle(&elf) + me.points()
                })
                .sum(),
        )
    }
}

fn main() {
    aoc::solution!(Day02)
}
#[cfg(test)]
mod tests {
    use crate::Day02 as day_02;
    use crate::*;

    aoc::test_common!(day_02);

    aoc::test! {
        day_02:
        - "A Y\nB X\nC Z"
            => Some(15)
            => Some(12)
    }
}

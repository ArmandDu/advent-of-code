use aoc::solution::SolutionError;
use aoc::Solution;
use std::collections::HashSet;

struct Day09;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
struct Knot(i32, i32);
struct Direction(i32, i32);
struct Instruction(Direction, i32);

impl TryFrom<&str> for Instruction {
    type Error = SolutionError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (dir, count) = line.split_once(' ').ok_or(SolutionError::ParseError)?;

        let count = count.parse().or(Err(SolutionError::ParseError))?;
        let dir = match dir {
            "R" => Direction(1, 0),
            "L" => Direction(-1, 0),
            "U" => Direction(0, -1),
            "D" => Direction(0, 1),
            _ => return Err(SolutionError::ParseError),
        };

        Ok(Instruction(dir, count))
    }
}

impl Knot {
    fn move_head(self, delta: &Direction) -> Self {
        Self(self.0 + delta.0, self.1 + delta.1)
    }

    fn move_body(self, to: &Self) -> Self {
        let (delta_x, delta_y) = (to.0 - self.0, to.1 - self.1);

        match (delta_x.abs(), delta_y.abs()) {
            (0, 1) | (1, 1) | (1, 0) | (0, 0) => self,
            _ => Self(self.0 + delta_x.signum(), self.1 + delta_y.signum()),
        }
    }
}

impl Day09 {
    fn move_rope<const SIZE: usize>(instructions: &[Instruction]) -> HashSet<Knot> {
        let (_, history) = instructions.iter().fold(
            ([Knot::default(); SIZE], HashSet::new()),
            |(mut body, mut history), Instruction(delta, count)| {
                for _ in 1..=*count {
                    for index in 0..SIZE {
                        body[index] = match index {
                            0 => body[0].move_head(delta),
                            _ => body[index].move_body(&body[index - 1]),
                        };
                    }

                    history.insert(*body.last().unwrap());
                }

                (body, history)
            },
        );

        history
    }
}

impl Solution for Day09 {
    const TITLE: &'static str = "Rope Bridge";
    const DAY: u8 = 9;
    type Input = Vec<Instruction>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Ok(input
            .lines()
            .filter_map(|line| Instruction::try_from(line).ok())
            .collect())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        Some(Day09::move_rope::<2>(input).len())
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        Some(Day09::move_rope::<10>(input).len())
    }
}

fn main() {
    aoc::solution!(Day09)
}
#[cfg(test)]
mod tests {
    use crate::Day09 as day_09;
    use crate::*;

    aoc::test_common!(day_09);

    aoc::test! {
     day_09:
     [case_1]
        - "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n" => Some(13) => Some(1);
     [case_2]
        - "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n" => Some(88) => Some(36);
    }
}

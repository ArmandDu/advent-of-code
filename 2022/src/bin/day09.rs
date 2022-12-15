use aoc::Solution;
use std::collections::HashSet;

struct Day09;

impl Day09 {
    fn move_head(head: (i32, i32), delta: &(i32, i32)) -> (i32, i32) {
        (head.0 + delta.0, head.1 + delta.1)
    }

    fn move_tail(from: (i32, i32), to: &(i32, i32)) -> (i32, i32) {
        let (delta_x, delta_y) = (to.0 - from.0, to.1 - from.1);

        match (delta_x.abs(), delta_y.abs()) {
            (0, 1) | (1, 1) | (1, 0) | (0, 0) => from,
            (x, y) if x <= 2 && y <= 2 => (from.0 + delta_x.signum(), from.1 + delta_y.signum()),
            (0, 2) => (from.0 + delta_x.signum(), from.1 + delta_y),
            (2, 0) => (from.0 + delta_x, from.1 + delta_y.signum()),
            _ => unreachable!(),
        }
    }

    fn move_rope(instructions: &[((i32, i32), i32)], rope_size: usize) -> HashSet<(i32, i32)> {
        let (_, history) = instructions.iter().fold(
            (vec![(0, 0); rope_size], HashSet::new()),
            |(mut body, mut history), (delta, count)| {
                for _ in 1..=*count {
                    for index in 0..rope_size {
                        body[index] = match index {
                            0 => Day09::move_head(body[0], delta),
                            _ => Day09::move_tail(body[index], &body[index - 1]),
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
    type Input = Vec<((i32, i32), i32)>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Ok(input
            .lines()
            .filter_map(|line| {
                let (dir, count) = line.split_once(' ')?;

                let dir = match dir {
                    "R" => Some((1, 0)),
                    "L" => Some((-1, 0)),
                    "U" => Some((0, -1)),
                    "D" => Some((0, 1)),
                    _ => None,
                };

                Some((dir?, count.parse().ok()?))
            })
            .collect())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        Some(Day09::move_rope(input, 2).len())
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        Some(Day09::move_rope(input, 10).len())
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

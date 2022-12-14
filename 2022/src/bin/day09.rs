use aoc::Solution;
use std::collections::HashMap;

struct Day09;

impl Day09 {
    fn move_tail(tail: (i32, i32), head: &(i32, i32), delta: &(i32, i32)) -> (i32, i32) {
        match (tail.0.abs_diff(head.0), tail.1.abs_diff(head.1)) {
            /*
            h = 2,0
            t = 0,0
            d = 1,0
            tx = 0 + (2-0) - 1 = 1,
            ty = 0 + (0-0) - 0 = 0

            h = 3,1
            t = 2,0
            d = 0,1
            tx = 2 + (3-2) - 0 = 3,
            ty = 0 + (1-0) - 1 = 0

            h = 2,2
            t = 0,0
            d = 1,0
            tx = 0 + (2 - 0) - 1 = 1,
            ty = 0 + (2 - 0) - 0 = 2
            */
            (0, n) | (n, 0) if n > 1 => (
                tail.0 + (head.0 - tail.0) - (head.0 - tail.0).signum(),
                tail.1 + (head.1 - tail.1) - (head.1 - tail.1).signum(),
            ),
            (1, 2) | (2, 1) => (
                tail.0 + (head.0 - tail.0) - delta.0,
                tail.1 + (head.1 - tail.1) - delta.1,
            ),

            _ => tail,
        }
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
        type ReducerState = ((i32, i32), (i32, i32), HashMap<(i32, i32), usize>);

        let (_, _, history) = input.iter().fold(
            ((0, 0), (0, 0), HashMap::new()),
            |(mut head, mut tail, mut history): ReducerState, (delta, count)| {
                for _step in 1..=*count {
                    head = (head.0 + delta.0, head.1 + delta.1);

                    tail = Day09::move_tail(tail, &head, delta);

                    *history.entry(tail).or_default() += 1;
                }

                (head, tail, history)
            },
        );

        Some(history.len())
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        type ReducerState = ([(i32, i32); 10], HashMap<(i32, i32), usize>);

        let (_, history) = input.iter().fold(
            ([(0, 0); 10], HashMap::new()),
            |(mut body, mut history): ReducerState, (delta, count)| {
                for _step in 1..=*count {
                    let head = body.first().unwrap();

                    body[0] = (head.0 + delta.0, head.1 + delta.1);

                    for index in 1..10 {
                        body[index] = Day09::move_tail(body[index], &body[index - 1], delta);
                    }

                    dbg!(&body);

                    *history.entry(*body.last().unwrap()).or_default() += 1;
                }

                (body, history)
            },
        );

        Some(history.len())
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

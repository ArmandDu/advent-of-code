use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};

use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;

type Point = (i64, i64);

struct Day17;

#[derive(Debug, Clone)]
struct Shape(&'static [Point]);

impl Display for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let bound_x = self.0.iter().map(|p| p.0).minmax().into_option().unwrap();
        let bound_y = self.0.iter().map(|p| p.1).minmax().into_option().unwrap();

        writeln!(
            f,
            "{}",
            (bound_y.0..=bound_y.1)
                .rev()
                .map(|y| (bound_x.0..=bound_x.1)
                    .map(|x| match self.0.contains(&(x, y)) {
                        true => "#",
                        _ => " ",
                    })
                    .join(""))
                .join("\n")
        )
    }
}

impl Shape {
    fn h_line() -> Self {
        Self(&[(0, 0), (1, 0), (2, 0), (3, 0)])
    }
    fn v_line() -> Self {
        Self(&[(0, 0), (0, 1), (0, 2), (0, 3)])
    }

    fn cross() -> Self {
        Self(&[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)])
    }

    fn rev_l() -> Self {
        Self(&[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)])
    }

    fn square() -> Self {
        Self(&[(0, 0), (1, 0), (0, 1), (1, 1)])
    }
}

impl Shape {
    fn iter(&self) -> impl Iterator<Item = &Point> {
        self.0.iter()
    }
}

impl Day17 {
    #[allow(unused)]
    fn print(cache: &HashSet<Point>, shape: Option<&Shape>, start: usize, height: usize) {
        println!(
            "{}\n+-------+",
            (start..height)
                .rev()
                .map(|y| format!(
                    "|{}| {}",
                    (0..7)
                        .map(|x| {
                            let p = (x, y as i64);

                            match cache.get(&p) {
                                Some(_) => "#",
                                _ => match shape {
                                    Some(s) => match s.0.contains(&p) {
                                        true => "@",
                                        _ => ".",
                                    },
                                    _ => ".",
                                },
                            }
                        })
                        .join(""),
                    y
                ))
                .join("\n")
        );
    }
}

impl Day17 {
    fn can_move(pos: &(i64, i64), shape: &Shape, map: &HashSet<Point>, offset: Point) -> bool {
        shape.iter().all(|point| {
            let next_x = point.0 + pos.0 + offset.0;
            let next_y = point.1 + pos.1 + offset.1;

            !map.contains(&(next_x, next_y)) && next_y >= 0 && (0..7).contains(&next_x)
        })
    }

    fn try_move(
        pos: &mut (i64, i64),
        shape: &Shape,
        map: &HashSet<Point>,
        offset: impl Into<Point>,
    ) {
        let offset = offset.into();

        if Self::can_move(pos, shape, map, offset) {
            pos.0 += offset.0;
            pos.1 += offset.1;
        }
    }

    fn fall(
        shape: &Shape,
        cache: &mut HashSet<Point>,
        height: &mut usize,
        mut move_factory: impl FnMut() -> i64,
    ) {
        let mut pos = (2, *height as i64 + 3);

        let mut is_first = true;
        while Day17::can_move(&pos, shape, cache, (0, -1)) {
            if is_first {
                is_first = false
            } else {
                Day17::try_move(&mut pos, shape, cache, (0, -1));
            }

            let shift = move_factory();
            Day17::try_move(&mut pos, shape, cache, (shift, 0));
        }

        shape.iter().for_each(|point| {
            let point = (point.0 + pos.0, point.1 + pos.1);

            cache.insert(point);
            *height = (*height).max((point.1 + 1) as usize);
        });
    }

    fn solve(target: usize, shapes: &[Shape], moves: &[i64]) -> Option<usize> {
        let mut moves_pos = 0;
        let mut count = 0;

        let mut height = 0;
        let mut cache = HashSet::new();
        let mut cycle = HashMap::new();
        let mut cycle_height = 0;

        while count < target {
            let shape = &shapes[count % shapes.len()];
            Day17::fall(shape, &mut cache, &mut height, || {
                let offset = moves[moves_pos];

                moves_pos = (moves_pos + 1) % moves.len();
                offset
            });

            // Check for Cycle
            if cycle_height == 0 {
                let cycle_key = (count % shapes.len(), moves_pos);

                if let Some((2, prev_height, prev_count)) = cycle.get(&cycle_key) {
                    let delta_height = height - prev_height;
                    let delta_count = count - prev_count;
                    let repeats = (target - count) / delta_count;

                    cycle_height = delta_height * repeats;
                    count += delta_count * repeats;
                }

                cycle
                    .entry(cycle_key)
                    .and_modify(|(cycle, prev_height, prev_count)| {
                        *cycle += 1;
                        *prev_count = count;
                        *prev_height = height;
                    })
                    .or_insert((1_usize, height, count));
            }

            count += 1;
        }

        Some(height + cycle_height)
    }
}

impl Solution for Day17 {
    const TITLE: &'static str = "Pyroclastic Flow";
    const DAY: u8 = 17;
    type Input = ([Shape; 5], Vec<i64>);
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        let moves = input
            .trim()
            .chars()
            .map(|c| match c {
                '<' => Ok(-1),
                '>' => Ok(1),
                _ => Err(SolutionError::ParseError),
            })
            .collect::<Result<Vec<_>, SolutionError>>()?;

        let shapes = [
            Shape::h_line(),
            Shape::cross(),
            Shape::rev_l(),
            Shape::v_line(),
            Shape::square(),
        ];

        Ok((shapes, moves))
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let (shapes, moves) = input;

        Day17::solve(2022, shapes, moves)
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let (shapes, moves) = input;

        Day17::solve(1_000_000_000_000, shapes, moves)
    }
}

fn main() {
    aoc::solution!(Day17)
}
#[cfg(test)]
mod tests {
    use crate::Day17 as day_17;
    use crate::*;

    aoc::test_common!(day_17);

    aoc::test! {
        day_17:
        - ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
            => Some(3068)
            => Some(1514285714288)
    }
}

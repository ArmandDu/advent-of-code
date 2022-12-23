use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

use std::str::FromStr;

use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::{EitherOrBoth, Itertools};

type Point = (i64, i64);

struct Day17;

#[derive(Debug, Clone)]
struct Shape(HashSet<Point>);

impl Hash for Shape {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.iter().for_each(|c| c.hash(state));
    }
}

impl PartialEq<Self> for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.iter().zip_longest(other.iter()).all(|zip| match zip {
            EitherOrBoth::Both(a, b) => a == b,
            _ => false,
        })
    }
}

impl Eq for Shape {}

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
                    .map(|x| match self.0.get(&(x, y)) {
                        Some(_) => "#",
                        None => " ",
                    })
                    .join(""))
                .join("\n")
        )
    }
}

impl Shape {
    fn h_line() -> Self {
        "####".parse().unwrap()
    }
    fn v_line() -> Self {
        "#\n#\n#\n#".parse().unwrap()
    }

    fn cross() -> Self {
        ".#.\n###\n.#.".parse().unwrap()
    }

    fn rev_l() -> Self {
        "###\n..#\n..#".parse().unwrap()
    }

    fn square() -> Self {
        "##\n##".parse().unwrap()
    }
}

impl Shape {
    fn iter(&self) -> impl Iterator<Item = &Point> {
        self.0.iter()
    }

    fn shift(&mut self, (x, y): Point) {
        self.0 = self
            .0
            .iter()
            .map(|dot| {
                let mut dot = *dot;
                dot.0 += x;
                dot.1 += y;

                dot
            })
            .collect();
    }
}

impl FromStr for Shape {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(HashSet::from_iter(s.lines().enumerate().flat_map(
            |(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| match c {
                    '#' => Some((x as i64, y as i64)),
                    _ => None,
                })
            },
        ))))
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
                                    Some(s) => match s.0.get(&p) {
                                        Some(_) => "@",
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
    fn can_move(shape: &Shape, map: &HashSet<Point>, offset: Point) -> bool {
        shape.iter().cloned().all(|mut point| {
            point.0 += offset.0;
            point.1 += offset.1;

            !map.contains(&point) && point.1 >= 0 && point.0 >= 0 && point.0 < 7
        })
    }

    fn try_move(shape: &mut Shape, map: &HashSet<Point>, offset: impl Into<Point>) {
        let offset = offset.into();

        if Self::can_move(shape, map, offset) {
            shape.shift(offset)
        }
    }

    fn fall(
        mut shape: Shape,
        mut cache: HashSet<Point>,
        mut height: usize,
        mut move_factory: impl FnMut() -> i64,
    ) -> (HashSet<Point>, usize) {
        shape.shift((2, (height as i64) + 3));

        let mut is_first = true;
        while Day17::can_move(&shape, &cache, (0, -1)) {
            if is_first {
                is_first = false
            } else {
                Day17::try_move(&mut shape, &cache, (0, -1));
            }

            let shift = move_factory();
            Day17::try_move(&mut shape, &cache, (shift, 0));
        }

        shape.iter().for_each(|point| {
            cache.insert(*point);
            height = height.max((point.1 + 1) as usize);
        });

        (cache, height)
    }
}

impl Solution for Day17 {
    const TITLE: &'static str = "Pyroclastic Flow";
    const DAY: u8 = 17;
    type Input = (Vec<Shape>, Vec<i64>);
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

        let shapes = vec![
            Shape::h_line(),
            Shape::cross(),
            Shape::rev_l(),
            Shape::v_line(),
            Shape::square(),
        ];

        Ok((shapes, moves))
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        const COUNT: usize = 2022;

        let (shapes, moves) = input;
        let shape_factory = shapes.iter().cycle();
        let mut moves_iter = moves.iter().cycle();

        let (_grid, height) = shape_factory.take(COUNT).cloned().fold(
            (HashSet::new(), 0),
            |(cache, height), shape| {
                Day17::fall(shape, cache, height, || *moves_iter.next().unwrap())
            },
        );

        Some(height)
    }

    fn part2(_input: &Self::Input) -> Option<Self::P2> {
        None
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

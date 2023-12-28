use std::str::FromStr;

use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;

type Point = (f64, f64, f64);
type Velocity = (f64, f64, f64);

#[derive(Debug)]
struct Hail {
    point: Point,
    velocity: Velocity,
    line: (f64, f64),
}

impl Hail {
    fn new((point, velocity): (Point, Velocity)) -> Self {
        let ((x, y, _), (a, b, _)) = (point, velocity);

        let s = b / a;

        Self {
            point,
            velocity,
            line: (s, y - s * x),
        }
    }

    fn x(&self) -> f64 {
        self.point.0
    }

    fn y(&self) -> f64 {
        self.point.1
    }

    fn vx(&self) -> f64 {
        self.velocity.0
    }
    fn vy(&self) -> f64 {
        self.velocity.1
    }

    fn intersects(&self, other: &Self) -> Option<(f64, f64)> {
        let (a, c) = self.line;
        let (b, d) = other.line;

        if a == b {
            return None;
        }

        let x = (d - c) / (a - b);
        let y = a * x + c;

        Some((x, y))
            .filter(|i| self.is_same_direction(i))
            .filter(|i| other.is_same_direction(i))
    }

    fn is_same_direction(&self, (x, y): &(f64, f64)) -> bool {
        let dir_x = (x - self.x()).signum();
        let dir_y = (y - self.y()).signum();

        dir_x == self.vx().signum() && dir_y == self.vy().signum()
    }
}

impl FromStr for Hail {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.replace('@', ",");

        input
            .split(", ")
            .filter_map(|point| point.trim().parse().ok())
            .chunks(3)
            .into_iter()
            .filter_map(|chunk| chunk.into_iter().collect_tuple())
            .collect_tuple()
            .ok_or(SolutionError::ParseError)
            .map(Hail::new)
    }
}

struct Day24<const MIN: usize, const MAX: usize>;

impl<const MIN: usize, const MAX: usize> Solution for Day24<MIN, MAX> {
    const TITLE: &'static str = "Never Tell Me The Odds";
    const DAY: u8 = 24;
    type Input = Vec<Hail>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        input.lines().map(Hail::from_str).collect()
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        Some(
            input
                .iter()
                .zip(1..)
                .flat_map(|(p1, i)| input[i..].iter().filter_map(|p2| p1.intersects(p2)))
                .filter(|(x, _)| MIN as f64 <= *x && *x <= MAX as f64)
                .filter(|(_, y)| MIN as f64 <= *y && *y <= MAX as f64)
                .count(),
        )
    }

    fn part2(_input: &Self::Input) -> Option<Self::P2> {
        None
    }
}

type Day24Main = Day24<200000000000000, 400000000000000>;

aoc::run!(Day24Main);

#[allow(dead_code)]
type Day24Test = Day24<7, 27>;

aoc::example! {
    [Day24Test]
    sample: "19, 13, 30 @ -2,  1, -2\r\n18, 19, 22 @ -1, -1, -2\r\n20, 25, 34 @ -2, -2, -4\r\n12, 31, 28 @ -1, -2, -1\r\n20, 19, 15 @  1, -5, -3\r\n"
        => Some(2)
        => None
}

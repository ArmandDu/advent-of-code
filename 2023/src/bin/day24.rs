use std::collections::HashSet;
use std::str::FromStr;

use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

type Point = (f64, f64, f64);
type Velocity = (f64, f64, f64);

#[derive(Debug)]
pub struct Hail {
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
    fn z(&self) -> f64 {
        self.point.2
    }

    fn vx(&self) -> f64 {
        self.velocity.0
    }
    fn vy(&self) -> f64 {
        self.velocity.1
    }
    fn vz(&self) -> f64 {
        self.velocity.2
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

pub struct Day24<const MIN: usize, const MAX: usize>;

impl<const MIN: usize, const MAX: usize> Day24<MIN, MAX> {
    fn brute_force_velocity(
        hails: &[Hail],
        get_p: impl Fn(&Hail) -> f64,
        get_v: impl Fn(&Hail) -> f64,
    ) -> Option<i128> {
        //range might be too narrow for other inputs.
        let range = -300..=200;

        //zip points for same velocity {hv = (p1, p2) }
        //solve (p2 - p1) % (rv - hv) = 0 for each sets
        // this will give multiples values for rv (rock's velocity)
        //given enough points, the set will reduce to a common rv for all sets
        //return that unique rv.
        let candidates = hails
            .iter()
            .sorted_by_key(|h| get_v(h) as i64)
            .group_by(|h| get_v(h))
            .into_iter()
            .filter_map(|(vh, group)| Some((vh, group.map(&get_p).collect_tuple::<(_, _)>()?)))
            .fold_while(HashSet::from_iter(range.clone()), |set, (vh, (a, b))| {
                let dist = b - a;

                let next_set: HashSet<_> = range
                    .clone()
                    .map(|v| v as f64)
                    .filter(|v| (dist % (v - vh)).abs() < f64::EPSILON)
                    .map(|v| v as i128)
                    .filter(|v| set.contains(v))
                    .collect();

                if next_set.len() == 1 {
                    Done(next_set)
                } else {
                    Continue(next_set)
                }
            })
            .into_inner();

        candidates
            .into_iter()
            .collect_tuple()
            .map(|(only_one,)| only_one)
    }
}

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
                .tuple_combinations()
                .filter_map(|(p1, p2)| p1.intersects(p2))
                .filter(|(x, _)| MIN as f64 <= *x && *x <= MAX as f64)
                .filter(|(_, y)| MIN as f64 <= *y && *y <= MAX as f64)
                .count(),
        )
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let vx = Self::brute_force_velocity(input, |h| h.x(), |h| h.vx())? as f64;
        let vy = Self::brute_force_velocity(input, |h| h.y(), |h| h.vy())? as f64;
        let vz = Self::brute_force_velocity(input, |h| h.z(), |h| h.vz())? as f64;

        let (a, b) = input
            .iter()
            .take(2)
            .map(|h| Hail::new((h.point, (h.vx() - vx, h.vy() - vy, h.vz() - vz))))
            .collect_tuple()?;

        let (x, y) = a.intersects(&b)?;
        let t = (x - a.x()) / a.vx();
        let z = a.z() + a.vz() * t;

        [x, y, z].iter().sum1().map(|sum: f64| sum as usize)
    }
}

pub type Day24Actual = Day24<200000000000000, 400000000000000>;

aoc::run!(Day24Actual);

#[allow(dead_code)]
type Day24Test = Day24<7, 27>;

aoc::example! {
    [Day24Test]
    sample: "19, 13, 30 @ -2,  1, -2\r\n18, 19, 22 @ -1, -1, -2\r\n20, 25, 34 @ -2, -2, -4\r\n12, 31, 28 @ -1, -2, -1\r\n20, 19, 15 @  1, -5, -3\r\n"
        => Some(2)
        => None
}

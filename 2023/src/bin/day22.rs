use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Clone)]
struct Point(usize, usize, usize);

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Clone)]
struct Brick(Point, Point);

impl Point {
    fn x(&self) -> usize {
        self.0
    }
    fn y(&self) -> usize {
        self.1
    }
    fn z(&self) -> usize {
        self.2
    }
}

impl Brick {
    fn move_z(&mut self, count: isize) {
        self.0 .2 = self.0.z().saturating_add_signed(count);
        self.1 .2 = self.1.z().saturating_add_signed(count);
    }

    fn intersect_xy(&self, other: &Self) -> bool {
        let Brick(l1, r1) = other;
        let Brick(l2, r2) = self;

        let overlaps_x = !(r1.x() < l2.x() || l1.x() > r2.x());
        let overlaps_y = !(r1.y() < l2.y() || l1.y() > r2.y());

        overlaps_x && overlaps_y
    }

    fn dist_z(&self, other: &Self) -> usize {
        self.1
            .z()
            .abs_diff(other.0.z())
            .min(self.0.z().abs_diff(other.1.z()))
    }
}

impl FromStr for Point {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .split(',')
            .filter_map(|coord| coord.parse().ok())
            .collect_tuple()
            .map(|(x, y, z)| Point(x, y, z))
            .ok_or(SolutionError::ParseError)
    }
}

impl FromStr for Brick {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('~').ok_or(SolutionError::ParseError)?;

        Ok(Brick(start.parse()?, end.parse()?))
    }
}

struct Day22;

type Child<'a> = (&'a Brick, Vec<&'a Brick>);

impl Day22 {
    fn overlaps(brick: &Brick, other: &Brick) -> bool {
        brick.intersect_xy(other)
    }

    fn tetris(input: &[Brick]) -> Vec<Brick> {
        let mut stack = vec![];

        for mut brick in input.iter().sorted_by_key(|b| b.0.z()).cloned() {
            while brick.0.z() > 0
                && !stack
                    .iter()
                    .any(|other| brick.intersect_xy(other) && brick.dist_z(other) == 1)
            {
                brick.move_z(-1);
            }

            stack.push(brick);
        }

        stack
    }

    fn dependencies(stack: &[Brick]) -> HashMap<&Brick, Vec<Child>> {
        stack
            .iter()
            .map(|brick| {
                (
                    brick,
                    stack
                        .iter()
                        .filter(|other| {
                            other.0.z() == brick.1.z() + 1 && Day22::overlaps(brick, other)
                        })
                        .map(|dep| {
                            (
                                dep,
                                stack
                                    .iter()
                                    .filter(|other| {
                                        (other.1.z() == dep.0.z() - 1)
                                            && Day22::overlaps(dep, other)
                                    })
                                    .collect_vec(),
                            )
                        })
                        .collect_vec(),
                )
            })
            .collect()
    }

    fn chain_reaction<'a>(
        source: &'a Brick,
        dependencies: &HashMap<&'a Brick, Vec<Child<'a>>>,
    ) -> HashSet<&'a Brick> {
        let mut stack = HashSet::new();
        let mut queue = VecDeque::new();

        stack.insert(source);
        queue.push_front(source);

        while let Some(brick) = queue.pop_front() {
            if let Some(children) = dependencies.get(brick) {
                for (child, parents) in children {
                    if parents.iter().all(|parent| stack.contains(parent)) {
                        queue.push_front(child);
                        stack.insert(child);
                    }
                }
            }
        }

        stack.remove(source);
        stack
    }
}

impl Solution for Day22 {
    const TITLE: &'static str = "Sand Slabs";
    const DAY: u8 = 22;
    type Input = Vec<Brick>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        input
            .lines()
            .map(Brick::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map(|bricks| {
                Day22::tetris(
                    &bricks
                        .into_iter()
                        .sorted_by_key(|brick| brick.0.z())
                        .collect_vec(),
                )
            })
    }

    fn part1(stack: &Self::Input) -> Option<Self::P1> {
        Some(
            Day22::dependencies(stack)
                .iter()
                .filter(|(base, children)| {
                    children.iter().all(|(_, parents)| {
                        parents.iter().filter(|parent| parent != base).count() > 0
                    })
                })
                .count(),
        )
    }

    fn part2(stack: &Self::Input) -> Option<Self::P2> {
        let dependencies = Day22::dependencies(stack);

        stack
            .iter()
            .map(|brick| Day22::chain_reaction(brick, &dependencies).len())
            .sum1()
    }
}

aoc::run!(Day22);

aoc::example! {
    [Day22]
    sample: "1,0,1~1,2,1\r\n0,0,2~2,0,2\r\n0,2,3~2,2,3\r\n0,0,4~0,2,4\r\n2,0,5~2,2,5\r\n0,1,6~2,1,6\r\n1,1,8~1,1,9\r\n"
        => Some(5)
        => Some(7)
}

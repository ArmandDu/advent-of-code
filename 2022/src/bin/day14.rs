use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::{Itertools, Product};
use std::fmt::{Display, Formatter};

use aoc_utils::dijkstra::Boundaries;
use itertools::FoldWhile::{Continue, Done};
use std::iter::{FlatMap, Map};
use std::ops::RangeInclusive;
use std::slice::Iter;

struct Day14;

#[derive(Debug, Copy, Clone, Default)]
struct Point(usize, usize);

impl TryFrom<&str> for Point {
    type Error = SolutionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use SolutionError::*;

        let (x, y) = value.split_once(',').ok_or(ParseError)?;
        let x = x.parse().or(Err(ParseError))?;
        let y = y.parse().or(Err(ParseError))?;

        Ok(Self(x, y))
    }
}

#[derive(Debug, Copy, Clone)]
struct Line(Point, Point);

impl From<(Point, Point)> for Line {
    fn from((start, end): (Point, Point)) -> Self {
        Self(start, end)
    }
}

type LineIter =
    Map<Product<RangeInclusive<usize>, RangeInclusive<usize>>, fn((usize, usize)) -> Point>;

impl Line {
    fn iter(&self) -> LineIter {
        let Self(start, end) = self;
        let start_x = start.0.min(end.0);
        let end_x = start.0.max(end.0);

        let start_y = start.1.min(end.1);
        let end_y = start.1.max(end.1);

        (start_x..=end_x)
            .cartesian_product(start_y..=end_y)
            .map(|(x, y)| Point(x, y))
    }
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    Sand,
    Rock,
}

#[derive(Debug, Clone)]
struct Grid {
    boundaries: Boundaries,
    tiles: Vec<Vec<Tile>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.tiles
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|tile| match tile {
                            Tile::Empty => ".",
                            Tile::Sand => "o",
                            Tile::Rock => "#",
                        })
                        .collect::<String>()
                })
                .join("\n")
        )
    }
}

impl TryFrom<&Vec<Line>> for Grid {
    type Error = SolutionError;

    fn try_from(lines: &Vec<Line>) -> Result<Self, Self::Error> {
        let (x, width) = lines
            .iter()
            .flat_map(|Line(start, end)| [start.0, end.0])
            .minmax()
            .into_option()
            .ok_or(SolutionError::ParseError)?;

        let height = lines
            .iter()
            .flat_map(|Line(start, end)| [start.1, end.1])
            .max()
            .ok_or(SolutionError::ParseError)?;

        let boundaries = Boundaries {
            x,
            y: 0,
            width,
            height,
        };

        let grid = Self {
            boundaries: boundaries.clone(),
            tiles: (boundaries.y..=boundaries.height)
                .map(|_| {
                    (boundaries.x..=boundaries.width)
                        .map(|_| Tile::Empty)
                        .collect()
                })
                .collect(),
        };

        let grid = lines
            .iter()
            .flat_map(|line| line.iter())
            .fold(grid, |mut grid, point| {
                if let Some(tile) = grid.get_mut(&point) {
                    *tile = Tile::Rock
                };

                grid
            });

        Ok(grid)
    }
}

type GridIter<'a> =
    FlatMap<Iter<'a, Vec<Tile>>, Iter<'a, Tile>, fn(&'a Vec<Tile>) -> Iter<'_, Tile>>;

impl Grid {
    fn iter(&self) -> GridIter {
        self.tiles.iter().flat_map(|row| row.iter())
    }

    fn get_mut(&mut self, point: &Point) -> Option<&mut Tile> {
        let x = point.0.checked_sub(self.boundaries.x)?;
        let y = point.1.checked_sub(self.boundaries.y)?;

        self.tiles.get_mut(y)?.get_mut(x)
    }

    fn get(&self, point: &Point) -> Option<&Tile> {
        let x = point.0.checked_sub(self.boundaries.x)?;
        let y = point.1.checked_sub(self.boundaries.y)?;

        self.tiles.get(y)?.get(x)
    }
}

impl Day14 {
    fn simulate(grid: Grid, leak: Point) -> Grid {
        (0..)
            .fold_while((grid, leak), |(mut grid, sand), _| {
                let down = Point(sand.0, sand.1 + 1);
                let left = Point(sand.0 - 1, sand.1 + 1);
                let right = Point(sand.0 + 1, sand.1 + 1);

                // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                // println!("{}", grid);
                // sleep(Duration::from_millis(1000 / 30));

                if let Some(Tile::Sand) = grid.get(&leak) {
                    return Done((grid, sand));
                }

                match (grid.get(&down), grid.get(&left), grid.get(&right)) {
                    (None, _, _) | (_, None, _) | (_, _, None) => Done((grid, sand)),
                    (Some(Tile::Empty), _, _) => Continue((grid, down)),
                    (_, Some(Tile::Empty), _) => Continue((grid, left)),
                    (_, _, Some(Tile::Empty)) => Continue((grid, right)),
                    (Some(Tile::Rock), _, _) | (Some(Tile::Sand), _, _) => {
                        *grid.get_mut(&sand).unwrap() = Tile::Sand;

                        Continue((grid, leak))
                    }
                }
            })
            .into_inner()
            .0
    }
}

impl Solution for Day14 {
    const TITLE: &'static str = "Regolith Reservoir";
    const DAY: u8 = 14;
    type Input = Vec<Line>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        let lines: Vec<Line> = input
            .lines()
            .flat_map(|line| {
                line.split(" -> ")
                    .filter_map(|point| Point::try_from(point).ok())
                    .tuple_windows::<(_, _)>()
            })
            .map(|line| line.into())
            .collect();

        Ok(lines)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        Some(
            Day14::simulate(input.try_into().ok()?, Point(500, 0))
                .iter()
                .filter(|tile| matches!(tile, Tile::Sand))
                .count(),
        )
    }

    fn part2(_input: &Self::Input) -> Option<Self::P2> {
        None
    }
}

fn main() {
    aoc::solution!(Day14)
}
#[cfg(test)]
mod tests {
    use crate::Day14 as day_14;
    use crate::*;

    aoc::test_common!(day_14);

    aoc::test! {
     day_14:
     [case_1]
        - "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n"
            => Some(24)
            => None;
    }
}

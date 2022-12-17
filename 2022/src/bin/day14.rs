use aoc::Solution;
use itertools::{Itertools, Product};
use std::collections::HashMap;
use std::env;

use crate::cave::{Cave, Tile};

use itertools::FoldWhile::{Continue, Done};

use shared::Point;
use std::iter::Map;
use std::ops::RangeInclusive;

struct Day14;

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
        let min = start.min(end);
        let max = start.max(end);

        (min.x()..=max.x())
            .cartesian_product(min.y()..=max.y())
            .map(Point::from)
    }
}

mod cave {
    use aoc::solution::SolutionError;
    use aoc_utils::dijkstra::Boundaries;
    use itertools::Itertools;
    use shared::Point;
    use std::collections::HashMap;
    use std::ops::{Deref, DerefMut};

    #[derive(Debug, Copy, Clone)]
    pub enum Tile {
        Sand,
        Rock,
    }

    #[derive(Debug, Clone)]
    pub struct Cave {
        boundaries: Boundaries,
        tiles: HashMap<Point, Tile>,
    }

    impl Cave {
        pub fn height(&self) -> usize {
            self.boundaries.height
        }
    }

    impl Deref for Cave {
        type Target = HashMap<Point, Tile>;

        fn deref(&self) -> &Self::Target {
            &self.tiles
        }
    }

    impl DerefMut for Cave {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.tiles
        }
    }

    impl Cave {
        pub fn get_tile(&self, point: &Point, bottom: Option<usize>) -> Option<&Tile> {
            match self.get(point) {
                Some(tile) => Some(tile),
                None => match (point.y(), bottom) {
                    (j, Some(depth)) if j == self.boundaries.height + depth => Some(&Tile::Rock),
                    _ => None,
                },
            }
        }

        pub fn print(&self, leak: Point, bottom: Option<usize>) -> String {
            let depth = bottom.unwrap_or(0);

            (self.boundaries.y..=self.boundaries.height + depth)
                .map(|j| {
                    (self.boundaries.x - depth..=self.boundaries.width + depth)
                        .map(|i| match Point::new(i, j) {
                            p if p == leak => '+',
                            p => match self.get_tile(&p, bottom) {
                                Some(Tile::Sand) => 'o',
                                Some(Tile::Rock) => '#',
                                None => '.',
                            },
                        })
                        .join("")
                })
                .join("\n")
        }
    }

    impl TryFrom<&HashMap<Point, Tile>> for Cave {
        type Error = SolutionError;

        fn try_from(tiles: &HashMap<Point, Tile>) -> Result<Self, Self::Error> {
            let (x, width) = tiles
                .keys()
                .map(|point| point.x())
                .minmax()
                .into_option()
                .ok_or(SolutionError::ParseError)?;

            let height = tiles
                .keys()
                .map(|point| point.y())
                .max()
                .ok_or(SolutionError::ParseError)?;

            let boundaries = Boundaries {
                x,
                y: 0,
                width,
                height,
            };

            Ok(Self {
                boundaries,
                tiles: tiles.to_owned(),
            })
        }
    }
}

impl Day14 {
    fn simulate(initial: Cave, leak: Point, bottom: Option<usize>) -> Cave {
        let max_y = initial.height() + bottom.unwrap_or_default();

        (0..)
            .fold_while((initial, leak), |(mut cave, sand), _| {
                if sand.y() >= max_y {
                    return Done((cave, sand));
                }

                if let Some(Tile::Sand) = cave.get(&leak) {
                    return Done((cave, sand));
                }

                let down = Point::new(sand.x(), sand.y() + 1);
                let left = Point::new(sand.x() - 1, sand.y() + 1);
                let right = Point::new(sand.x() + 1, sand.y() + 1);

                match (
                    cave.get_tile(&down, bottom),
                    cave.get_tile(&left, bottom),
                    cave.get_tile(&right, bottom),
                ) {
                    (None, _, _) => Continue((cave, down)),
                    (_, None, _) => Continue((cave, left)),
                    (_, _, None) => Continue((cave, right)),
                    (Some(_), _, _) => {
                        cave.insert(sand, Tile::Sand);

                        Continue((cave, leak))
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
    type Input = HashMap<Point, Tile>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Ok(input
            .lines()
            .flat_map(|line| {
                line.split(" -> ")
                    .filter_map(|point| Point::try_from(point).ok())
                    .tuple_windows::<(_, _)>()
            })
            .flat_map(|line| Line::from(line).iter().map(|point| (point, Tile::Rock)))
            .collect())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let cave = Cave::try_from(input).ok()?;
        let leak = Point::new(500, 0);
        let rock_count = cave.len();

        Some(Day14::simulate(cave, leak, None).len() - rock_count)
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let cave = Cave::try_from(input).ok()?;
        let leak = Point::new(500, 0);
        let rock_count = cave.len();

        Some(Day14::simulate(cave, leak, Some(2)).len() - rock_count)
    }
}

fn main() {
    match env::args().nth(1) {
        Some(query) if query == "--print" => {
            let input = Day14::parse(&Day14::get_input().unwrap()).unwrap();
            let cave = Cave::try_from(&input).unwrap();
            let leak = Point::new(500, 0);

            println!("==PART 1===");
            println!("{}", cave.print(leak, None));
            println!("\n==PART 2===");
            println!("{}", cave.print(leak, Some(2)));
        }
        _ => {}
    }
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
            => Some(93);
    }
}

use std::str::FromStr;

use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;

use crate::canvas::{Line, Point};
use crate::viz::print;

mod viz {
    use std::env::args;
    use std::ops::RangeInclusive;

    use itertools::Itertools;

    use crate::canvas::{Line, Point};
    use crate::DigSite;

    pub fn print() -> bool {
        args().any(|c| c.contains("--print"))
    }

    impl Line {
        fn contains(&self, point: &Point) -> bool {
            self.intersects_x(point.x()) && self.intersects_y(point.y())
        }

        fn intersects_x(&self, x: isize) -> bool {
            let x0 = self.start().x().min(self.end().x());
            let x1 = self.start().x().max(self.end().x());

            x0 <= x && x <= x1
        }
        fn intersects_y(&self, y: isize) -> bool {
            let y0 = self.start().y().min(self.end().y());
            let y1 = self.start().y().max(self.end().y());

            y0 <= y && y <= y1
        }
    }

    impl DigSite {
        fn range_y(&self) -> RangeInclusive<isize> {
            let (start, end) = self
                .0
                .iter()
                .flat_map(|line| [line.start().y(), line.end().y()])
                .minmax()
                .into_option()
                .map(|(y0, y1)| (y0, y1))
                .unwrap_or_default();

            start..=end
        }
        fn range_x(&self) -> RangeInclusive<isize> {
            let (start, end) = self
                .0
                .iter()
                .flat_map(|line| [line.start().x(), line.end().x()])
                .minmax()
                .into_option()
                .map(|(x0, x1)| (x0, x1))
                .unwrap_or_default();

            start..=end
        }
        pub fn print(&self) {
            for y in self.range_y() {
                for x in self.range_x() {
                    match self.0.iter().any(|line| line.contains(&Point::new(x, y))) {
                        true => print!("█"),
                        _ => print!(" "),
                    }
                }
                println!()
            }
            println!();
        }
    }
}

mod canvas {
    #[derive(Debug, Default, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
    pub struct Point(isize, isize);

    #[derive(Default, Debug, Hash, Eq, PartialEq, Copy, Clone)]
    pub struct Line(Point, Point);

    impl Point {
        pub fn new(x: isize, y: isize) -> Self {
            Self(x, y)
        }
        pub fn x(&self) -> isize {
            self.0
        }

        pub fn y(&self) -> isize {
            self.1
        }
    }

    impl Line {
        pub fn new(start: Point, end: Point) -> Self {
            Self(start, end)
        }
        pub fn dist(&self) -> usize {
            let Line(Point(x0, y0), Point(x1, y1)) = self;

            (x1.abs_diff(*x0)) + (y1.abs_diff(*y0))
        }

        pub fn start(&self) -> &Point {
            &self.0
        }
        pub fn end(&self) -> &Point {
            &self.1
        }
    }
}

struct Day18;

#[derive(Debug)]
struct Instruction(String, isize, String);

#[derive(Debug, Default)]
struct DigSite(Vec<Line>);

impl Instruction {
    fn by_values(&self) -> ((isize, isize), isize) {
        (
            match &self.0[..] {
                "D" => (0, 1),
                "U" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => unreachable!(),
            },
            self.1,
        )
    }

    fn by_color(&self) -> Option<((isize, isize), isize)> {
        let size = isize::from_str_radix(&self.2[1..6], 16).ok()?;

        let dir = match &self.2[6..] {
            "0" => (1, 0),
            "1" => (0, 1),
            "2" => (-1, 0),
            "3" => (0, -1),
            _ => unreachable!(),
        };

        Some((dir.to_owned(), size))
    }
}

impl DigSite {
    fn new(instructions: &[((isize, isize), isize)]) -> Self {
        Self(
            instructions
                .iter()
                .fold(((0, 0), vec![]), |(start, mut lines), ((dx, dy), size)| {
                    let end = (start.0 + dx * size, start.1 + dy * size);

                    lines.push(Line::new(
                        Point::new(start.0, start.1),
                        Point::new(end.0, end.1),
                    ));

                    (end, lines)
                })
                .1,
        )
    }

    fn border_size(&self) -> usize {
        self.0.iter().map(|line| line.dist()).sum()
    }

    fn area(&self) -> Option<usize> {
        self.0
            .iter()
            .map(|line| line.start().x() * line.end().y() - line.end().x() * line.start().y())
            .sum1::<isize>()
            .map(|area| area.unsigned_abs())
            .map(|area| area + self.border_size())
            .map(|area| area / 2 + 1)
    }
}

impl FromStr for Instruction {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, size, color) = s
            .split_whitespace()
            .collect_tuple()
            .ok_or(SolutionError::ParseError)?;
        let size = size.parse().map_err(|_| SolutionError::ParseError)?;

        Ok(Instruction(
            dir.to_owned(),
            size,
            color.replace(['(', ')'], ""),
        ))
    }
}

impl Solution for Day18 {
    const TITLE: &'static str = "Lavaduct Lagoon";
    const DAY: u8 = 18;
    type Input = Vec<Instruction>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        input.lines().map(Instruction::from_str).collect()
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let dig_site = DigSite::new(&input.iter().map(Instruction::by_values).collect_vec());

        if print() {
            dig_site.print();
        }

        dig_site.area()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        DigSite::new(&input.iter().filter_map(Instruction::by_color).collect_vec()).area()
    }
}

aoc::run!(Day18);

aoc::example! {
    [Day18]
    sample: "R 6 (#70c710)\r\nD 5 (#0dc571)\r\nL 2 (#5713f0)\r\nD 2 (#d2c081)\r\nR 2 (#59c680)\r\nD 2 (#411b91)\r\nL 5 (#8ceee2)\r\nU 2 (#caa173)\r\nL 1 (#1b58a2)\r\nU 2 (#caa171)\r\nR 2 (#7807d2)\r\nU 3 (#a77fa3)\r\nL 2 (#015232)\r\nU 2 (#7a21e3)\r\n"
        => Some(62)
        => Some(952408144115)
}

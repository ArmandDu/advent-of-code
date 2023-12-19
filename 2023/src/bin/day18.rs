use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::env::args;
use std::ops::Range;
use std::str::FromStr;

fn print() -> bool {
    args().any(|c| c.contains("--print"))
}

struct Day18;

#[derive(Debug)]
struct DigInstruction(String, usize, String);

impl DigInstruction {
    fn part1(&self) -> (String, usize) {
        (self.0.to_owned(), self.1)
    }
}

#[derive(Debug, Default)]
struct DigSite {
    borders: HashSet<(isize, isize)>,
    range_x: (isize, isize),
    range_y: (isize, isize),
}

impl DigSite {
    fn new(instructions: &[(String, usize)]) -> Self {
        let borders = Self::dig(instructions);

        Self {
            range_x: borders
                .iter()
                .map(|(x, _)| *x)
                .minmax()
                .into_option()
                .map(|(x0, x1)| (x0 - 1, x1 + 1))
                .unwrap_or_default(),
            range_y: borders
                .iter()
                .map(|(_, y)| *y)
                .minmax()
                .into_option()
                .map(|(y0, y1)| (y0 - 1, y1 + 1))
                .unwrap_or_default(),
            borders,
        }
    }

    fn range_y(&self) -> Range<isize> {
        let (start, end) = self.range_y;

        start..end
    }
    fn range_x(&self) -> Range<isize> {
        let (start, end) = self.range_x;

        start..end
    }

    fn dig(instructions: &[(String, usize)]) -> HashSet<(isize, isize)> {
        let mut area = HashSet::new();
        let mut pos = (0_isize, 0_isize);

        for (dir, size) in instructions {
            let (dx, dy) = match &dir[..] {
                "D" => (0, 1),
                "U" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => unreachable!(),
            };

            for _ in 0..*size {
                pos = (pos.0 + dx, pos.1 + dy);
                area.insert(pos);
            }
        }
        area
    }

    fn border_size(&self) -> usize {
        self.borders.len()
    }

    fn pool(&self) -> Option<HashSet<(isize, isize)>> {
        let start = self.range_y().find_map(|y| {
            let x = self
                .borders
                .iter()
                .filter_map(|(kx, ky)| (ky == &y).then_some(kx))
                .min()?
                + 1;

            (!self.borders.contains(&(x, y))).then_some((x, y))
        })?;

        let mut history = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_front(start);

        while let Some(current) = queue.pop_front() {
            if self.borders.contains(&current) {
                continue;
            }

            if !history.insert(current) {
                continue;
            }

            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next = (current.0 + dx, current.1 + dy);

                queue.push_front(next);
            }
        }

        Some(history)
    }

    fn print(&self, pool: &HashSet<(isize, isize)>) {
        for y in self.range_y() {
            for x in self.range_x() {
                match pool.contains(&(x, y)) {
                    true => print!("o"),
                    _ => match self.borders.get(&(x, y)) {
                        Some(_) => print!("#"),
                        _ => print!("."),
                    },
                }
            }
            println!()
        }
        println!();
    }
}

impl FromStr for DigInstruction {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, size, color) = s
            .split_whitespace()
            .collect_tuple()
            .ok_or(SolutionError::ParseError)?;
        let size = size.parse().map_err(|_| SolutionError::ParseError)?;

        Ok(DigInstruction(
            dir.to_owned(),
            size,
            color.replace(['(', ')'], ""),
        ))
    }
}

impl Solution for Day18 {
    const TITLE: &'static str = "Lavaduct Lagoon";
    const DAY: u8 = 18;
    type Input = Vec<DigInstruction>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        input.lines().map(DigInstruction::from_str).collect()
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let dig_site = DigSite::new(&input.iter().map(|inst| inst.part1()).collect_vec());
        let pool = dig_site.pool()?;
        let size = dig_site.border_size();

        if print() {
            dig_site.print(&pool);
        }

        Some(pool.len() + size)
    }

    fn part2(_input: &Self::Input) -> Option<Self::P2> {
        None
    }
}

aoc::run!(Day18);

aoc::example! {
    [Day18]
    sample: "R 6 (#70c710)\r\nD 5 (#0dc571)\r\nL 2 (#5713f0)\r\nD 2 (#d2c081)\r\nR 2 (#59c680)\r\nD 2 (#411b91)\r\nL 5 (#8ceee2)\r\nU 2 (#caa173)\r\nL 1 (#1b58a2)\r\nU 2 (#caa171)\r\nR 2 (#7807d2)\r\nU 3 (#a77fa3)\r\nL 2 (#015232)\r\nU 2 (#7a21e3)\r\n"
        => Some(62)
        => None
}

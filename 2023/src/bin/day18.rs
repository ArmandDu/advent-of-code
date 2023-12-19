use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

struct Day18;

#[derive(Debug)]
struct DigInstruction(String, usize, String);

impl DigInstruction {
    fn part1(&self) -> (String, usize) {
        (self.0.to_owned(), self.1)
    }
}

#[derive(Debug, Default)]
struct DigSite(HashSet<(isize, isize)>);

impl DigSite {
    fn new(instructions: &[(String, usize)]) -> Self {
        let mut dig = Self::default();

        dig.dig(instructions);

        dig
    }

    fn width(&self) -> isize {
        self.0.iter().map(|(x, _)| x + 1).max().unwrap_or_default()
    }
    fn height(&self) -> isize {
        self.0.iter().map(|(_, y)| y + 1).max().unwrap_or_default()
    }
    fn dig(&mut self, instructions: &[(String, usize)]) {
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
                self.0.insert(pos);
            }
        }
    }

    fn border_size(&self) -> usize {
        self.0.len()
    }

    fn pool(&self) -> Option<HashSet<(isize, isize)>> {
        let height = self.height();

        let start = (0..height).find_map(|y| {
            let x = self
                .0
                .iter()
                .filter_map(|(kx, ky)| (ky == &y).then_some(kx))
                .min()?
                + 1;

            (!self.0.contains(&(x, y))).then_some((x, y))
        })?;

        let mut history = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_front(start);

        while let Some(current) = queue.pop_front() {
            if self.0.contains(&current) {
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

    #[allow(dead_code)]
    fn print(&self, pool: &HashSet<(isize, isize)>) {
        let width = self.width();
        let height = self.height();

        for y in 0..height {
            for x in 0..width {
                match pool.contains(&(x, y)) {
                    true => print!("o"),
                    _ => match self.0.get(&(x, y)) {
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

        // dig_site.print(&pool);

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

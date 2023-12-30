use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

struct Day14;

#[derive(Debug, Clone)]
struct Platform(Vec<Vec<char>>);
impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(|row| row.iter().join(" ")).join("\n")
        )
    }
}

impl Platform {
    fn new(maze: Vec<Vec<char>>) -> Self {
        Self(maze)
    }

    fn flip(&mut self) {
        let width = self.0.first().map(|r| r.len()).unwrap_or_default();
        let height = self.0.len();

        self.0 = (0..height)
            .map(|y| (0..width).map(|x| self.0[x][y]).collect())
            .collect();
    }

    fn tilt(&mut self, dir: char) {
        match dir {
            'N' | 'S' => self.flip(),
            _ => {}
        }

        self.0.iter_mut().for_each(|row| {
            const WALL: char = '#';
            let str: String = row.iter().collect();

            *row = itertools::intersperse(
                str.split(WALL).map(|c| match dir {
                    'N' | 'W' => c.chars().sorted().rev().collect_vec(),
                    _ => c.chars().sorted().collect_vec(),
                }),
                vec![WALL],
            )
            .flatten()
            .collect()
        });

        match dir {
            'N' | 'S' => self.flip(),
            _ => {}
        }
    }

    fn load(&self) -> usize {
        let height = self.0.len();

        self.0
            .iter()
            .enumerate()
            .map(|(y, row)| (height - y) * row.iter().filter(|c| c == &&'O').count())
            .sum()
    }
}

impl FromStr for Platform {
    type Err = SolutionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Platform::new(
            input.lines().map(|line| line.chars().collect()).collect(),
        ))
    }
}

impl Solution for Day14 {
    const TITLE: &'static str = "Parabolic Reflector Dish";
    const DAY: u8 = 14;
    type Input = Platform;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Platform::from_str(input)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let mut platform = input.clone();

        platform.tilt('N');

        Some(platform.load())
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        const CYCLES: usize = 1_000_000_000;
        let mut loads = vec![];
        let mut platform = input.to_owned();
        let mut cache = HashMap::new();

        (0..).find_map(|cycle| {
            for tilt in ['N', 'W', 'S', 'E'] {
                platform.tilt(tilt);
            }

            loads.push(platform.load());

            if let Some(offset) = cache.insert(platform.to_string(), cycle) {
                let size = cycle - offset;
                let target = (CYCLES - offset) % size + offset - 1;

                return Some(loads[target]);
            }

            None
        })
    }
}

aoc::run!(Day14);

aoc::example! {
    [Day14]
    simple: "O....#....\r\nO.OO#....#\r\n.....##...\r\nOO.#O....O\r\n.O.....O#.\r\nO.#..O.#.#\r\n..O..#O..O\r\n.......O..\r\n#....###..\r\n#OO..#....\r\n"
        => Some(136)
        => Some(64)
}

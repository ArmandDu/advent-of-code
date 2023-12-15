use aoc::solution::SolutionError;
use aoc::Solution;
use aoc_utils::collections::Matrix;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

struct Day14;

#[derive(Debug, Clone)]
struct Platform(Matrix<char>);
impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Platform {
    fn new(maze: Matrix<char>) -> Self {
        Self(maze)
    }

    fn tilt(&mut self, dir: char) {
        let (from, to) = match dir {
            'N' | 'W' => (".O", "O."),
            _ => ("O.", ".O"),
        };

        let rows: Vec<(usize, String)> = match dir {
            'N' | 'S' => self
                .0
                .iter_col()
                .map(|(x, col)| (x, col.map(|(_, c)| c).join("")))
                .collect_vec(),
            _ => self
                .0
                .iter_row()
                .map(|(y, col)| (y, col.map(|(_, c)| c).join("")))
                .collect_vec(),
        };

        rows.into_iter().for_each(|(y, mut row)| {
            while row.contains(from) {
                row = row.replace(from, to);
            }

            row.chars().enumerate().for_each(|(x, new)| {
                let (x, y) = match dir {
                    'N' | 'S' => (y, x),
                    _ => (x, y),
                };

                if let Some(origin) = self.0.get_mut(&(x, y)) {
                    *origin = new;
                }
            });
        });
    }

    fn load(&self) -> usize {
        let height = self.0.height();

        self.0
            .iter()
            .map(|((_, y), c)| if c == &'O' { height - y } else { 0 })
            .sum()
    }
}

impl FromStr for Platform {
    type Err = SolutionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Platform::new(input.into()))
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

    fn part2(_input: &Self::Input) -> Option<Self::P2> {
        None
    }
}

aoc::run!(Day14);

aoc::example! {
    [Day14]
    simple: "O....#....\r\nO.OO#....#\r\n.....##...\r\nOO.#O....O\r\n.O.....O#.\r\nO.#..O.#.#\r\n..O..#O..O\r\n.......O..\r\n#....###..\r\n#OO..#....\r\n"
        => Some(136)
        => None
}

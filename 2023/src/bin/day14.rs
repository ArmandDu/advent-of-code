use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

struct Day14;

#[derive(Debug, Clone)]
struct Platform {
    rocks: Vec<(usize, usize)>,
    maze: Vec<Vec<bool>>,
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let width = self.width();
        let header = (0..width).join(" ");

        write!(
            f,
            "    {header}\n{}",
            self.maze
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    format!(
                        "{y:#3} {}",
                        row.iter()
                            .enumerate()
                            .map(|(x, rock)| match (self.rocks.contains(&(x, y)), *rock) {
                                (true, true) => '?',
                                (true, _) => 'o',
                                (_, true) => '#',
                                _ => '.',
                            })
                            .join(" ")
                    )
                })
                .join("\n")
        )
    }
}

impl Platform {
    fn new(raw: Vec<Vec<char>>) -> Self {
        let rocks = raw
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, row)| row == &&'O')
                    .map(move |(x, _)| (x, y))
            })
            .collect();

        let maze = raw
            .iter()
            .map(|row| row.iter().map(|c| c == &'#').collect())
            .collect();

        Self { rocks, maze }
    }

    fn get(&self, x: usize, y: usize) -> Option<&bool> {
        self.maze.get(y)?.get(x)
    }

    fn tilt(mut self, dir: (isize, isize)) -> Self {
        let width = self.width() as isize;
        let height = self.height() as isize;

        let mut result = HashSet::new();

        let (off_x, off_y) = dir;
        for (x, y) in &self.rocks {
            if let Some((mut col_x, mut col_y)) = (1..).find_map(|delta| {
                let delta_x = *x as isize + (off_x * delta);
                let delta_y = *y as isize + (off_y * delta);

                match (delta_x, delta_y) {
                    (-1, y) => Some((0, y)),
                    (x, -1) => Some((x, 0)),
                    (x, y) if x >= width => Some((width - 1, y)),
                    (x, y) if y >= height => Some((x, height - 1)),
                    (x, y) if self.get(x as usize, y as usize) == Some(&true) => {
                        Some((x - off_x, y - off_y))
                    }
                    _ => None,
                }
                .map(|(x, y)| (x as usize, y as usize))
            }) {
                while result.contains(&(col_x, col_y)) {
                    col_x = col_x.saturating_add_signed(-off_x);
                    col_y = col_y.saturating_add_signed(-off_y);
                }

                result.insert((col_x, col_y));
            }
        }

        self.rocks = result.into_iter().collect();

        self
    }

    fn width(&self) -> usize {
        self.maze.first().map(|row| row.len()).unwrap_or_default()
    }

    fn height(&self) -> usize {
        self.maze.len()
    }

    fn load(&self) -> usize {
        let height = self.height();

        self.rocks.iter().map(|(_, y)| height - y).sum()
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
        let platform = input.clone();

        Some(platform.tilt((0, -1)).load())
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

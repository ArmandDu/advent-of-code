use std::collections::HashMap;
use std::str::FromStr;

use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;
use regex::Regex;

use dir::Dir;
use jungle::Jungle;

use crate::jungle::{Navigate, Render};
use crate::part1::SphereJungle;

mod dir;
mod jungle;
mod part1;

pub fn index(value: isize, min: isize, max: isize) -> usize {
    let size_y = max.abs_diff(min) as isize;

    (min + (max + (value % size_y)) % size_y) as usize
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tile {
    Open,
    Solid,
}

#[derive(Debug, Clone)]
enum Instruction {
    Forward(usize),
    Left,
    Right,
}

impl FromStr for Instruction {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Instruction::Right),
            "L" => Ok(Instruction::Left),
            digit => digit
                .parse()
                .map_err(|_| SolutionError::ParseError)
                .map(Instruction::Forward),
        }
    }
}

struct Day22;

impl Day22 {
    fn solve(
        mut current: (usize, usize),
        jungle: impl Navigate,
        instructions: &Vec<Instruction>,
        path: &mut HashMap<(usize, usize), Dir>,
    ) -> Option<usize> {
        let mut dir = Dir::default();

        for inst in instructions {
            match inst {
                Instruction::Forward(n) => {
                    let mut count = 0;

                    while count < *n && jungle.get(current, dir) != Some(&Tile::Solid) {
                        path.insert(current, dir);

                        (current, dir) = jungle.pos(current, dir)?;
                        count += 1;
                    }
                    path.insert(current, dir);
                }
                Instruction::Left => dir = dir.left(),
                Instruction::Right => dir = dir.right(),
            }
        }

        let row = (current.1 + 1) * 1000;
        let col = (current.0 + 1) * 4;
        let dir = usize::from(dir);

        Some(row + col + dir)
    }
}

impl Solution for Day22 {
    const TITLE: &'static str = "Monkey Map";
    const DAY: u8 = 22;
    type Input = (Jungle, Vec<Instruction>);
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        let lines = input.lines().collect_vec();
        let re = Regex::new(r"(L|R|\d+)").unwrap();

        let (grid, instructions) = lines
            .split(|line| line.is_empty())
            .collect_tuple::<(_, _)>()
            .ok_or(SolutionError::ParseError)?;

        let instructions = re
            .captures_iter(&instructions.join(""))
            .map(|captures| captures.get(1).map_or("", |m| m.as_str()).parse())
            .collect::<Result<_, _>>()?;

        let jungle = Jungle::from_str(&grid.join("\n"))?;

        Ok((jungle, instructions))
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let (jungle, instructions) = input;
        let mut path = HashMap::new();

        let score = Day22::solve(
            jungle.top_left()?,
            SphereJungle::new(jungle),
            instructions,
            &mut path,
        );
        is_verbose().then(|| println!("{}", Render::new(jungle, &path)));

        score
    }

    fn part2(_input: &Self::Input) -> Option<Self::P2> {
        None
    }
}

fn is_verbose() -> bool {
    std::env::args().any(|arg| arg.as_str() == "--print")
}

fn main() {
    aoc::solution!(Day22)
}
#[cfg(test)]
mod tests {
    use crate::Day22 as day_22;
    use crate::*;

    aoc::test_common!(day_22);

    aoc::test! {
        day_22:
        - "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.\n\n10R5L5R10L4R5L5\n"
            => Some(6032)
            => None
    }
}

use aoc::solution::SolutionError;
use aoc::Solution;
use aoc_utils::{index, lcm, pathfinding};
use itertools::Itertools;
use std::collections::HashSet;

use std::str::FromStr;

struct Day24;

struct Valley {
    map: Vec<HashSet<(usize, usize)>>,
    width: usize,
    height: usize,
    depth: usize,
}

impl FromStr for Valley {
    type Err = SolutionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();

        let storm: Vec<_> = input
            .lines()
            .enumerate()
            .skip(1)
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .skip(1)
                    .map(move |(x, char)| (x - 1, y - 1, char))
                    .filter_map(|(x, y, char)| match char {
                        '>' => Some(((x, y), (1, 0))),
                        '^' => Some(((x, y), (0, -1))),
                        '<' => Some(((x, y), (-1, 0))),
                        'v' => Some(((x, y), (0, 1))),
                        _ => None,
                    })
            })
            .collect();

        let (x_size, y_size) = (
            input.lines().next().ok_or(SolutionError::ParseError)?.len() - 2,
            input.lines().count() - 2,
        );

        let z_size = lcm(x_size, y_size).ok_or(SolutionError::ParseError)?;

        let valley = (0..z_size)
            .map(|z| {
                storm
                    .iter()
                    .map(|&((x, y), (dx, dy))| {
                        let (x, y, z) = (x as isize, y as isize, z as isize);
                        let (x_size, y_size) = (x_size as isize, y_size as isize);

                        let next_x = index(x + dx * z, 0, x_size);
                        let next_y = index(y + dy * z, 0, y_size);

                        (next_x, next_y)
                    })
                    .collect()
            })
            .collect();

        Ok(Self {
            map: valley,
            width: x_size,
            depth: y_size,
            height: z_size,
        })
    }
}

impl Valley {
    fn to_string(&self, z: usize, current: Option<(usize, usize)>) -> String {
        let floor = self.map.get(z % self.height).unwrap();

        let start = current.filter(|&(x, y)| self.is_start(&(x as i32, y as i32, 0)));
        let exit = current.filter(|&(x, y)| self.is_exit(&(x as i32, y as i32, 0)));

        format!(
            "floor {}:\n#{start}{pad}#\n{middle}\n#{pad}{exit}#",
            z,
            start = start.map_or('.', |_| 'E'),
            exit = exit.map_or('.', |_| 'E'),
            pad = std::iter::repeat('#').take(self.width - 1).join(""),
            middle = (0..self.depth)
                .map(|y| format!(
                    "#{}#",
                    (0..self.width)
                        .map(|x| {
                            if current.filter(|&(cx, cy)| cx == x && cy == y).is_some() {
                                match floor.contains(&(x, y)) {
                                    true => 'X',
                                    false => 'E',
                                }
                            } else {
                                match floor.contains(&(x, y)) {
                                    true => '+',
                                    false => '.',
                                }
                            }
                        })
                        .join("")
                ))
                .join("\n")
        )
    }

    fn exit(&self) -> (i32, i32) {
        let exit_x = self.width as i32;
        let exit_y = self.depth as i32 - 1;

        (exit_x, exit_y)
    }

    fn start(&self) -> (i32, i32) {
        (0, -1)
    }

    fn is_start(&self, (x, y, _): &(i32, i32, i32)) -> bool {
        let (start_x, start_y) = self.start();

        x == &start_x && y == &start_y
    }

    fn is_exit(&self, (x, y, _): &(i32, i32, i32)) -> bool {
        let (exit_x, exit_y) = self.exit();

        x == &exit_x && y == &exit_y
    }

    fn adjacent(&self, (x, y, z): &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
        let next_z = *z as usize + 1;

        self.map.get(next_z % self.height).map_or(vec![], |floor| {
            [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)]
                .iter()
                .filter_map(|&(dx, dy)| {
                    let next_x = x.checked_add(dx)?;
                    let next_y = y.checked_add(dy)?;

                    Some((next_x, next_y))
                })
                .filter(|&(x, y)| {
                    self.is_start(&(x, y, 0))
                        || self.is_exit(&(x, y, 0))
                        || 0 <= x && x < self.width as i32 && 0 <= y && y < self.depth as i32
                })
                .filter_map(|(next_x, next_y)| {
                    match !floor.contains(&(next_x as usize, next_y as usize)) {
                        true => Some((next_x, next_y, next_z as i32)),
                        false => None,
                    }
                })
                .collect()
        })
    }
}

impl Solution for Day24 {
    const TITLE: &'static str = "Blizzard Basin";
    const DAY: u8 = 24;
    type Input = Valley;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        input.parse()
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let start = (0, -1, 0);

        let path = pathfinding::bfs(
            &start,
            |coord| input.adjacent(coord).into_iter(),
            |coord| input.is_exit(coord),
        )?;

        is_print().then(|| {
            path.iter().for_each(|(x, y, z)| {
                println!(
                    "{}\n{:?}\n",
                    input.to_string(*z as usize, Some((*x as usize, *y as usize))),
                    (x, y, z)
                );
            })
        });

        Some(path.len() - 1)
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let start = input.start();
        let exit = input.exit();

        (0..3).fold(Some(0), |time, count| {
            let time = time.unwrap_or_default() as i32;
            let to_exit = count % 2 == 0;

            let path = pathfinding::bfs(
                &match to_exit {
                    true => (start.0, start.1, time),
                    _ => (exit.0, exit.1, time),
                },
                |coord| input.adjacent(coord).into_iter(),
                |coord| match to_exit {
                    true => input.is_exit(coord),
                    _ => input.is_start(coord),
                },
            );

            path.map(|path| path.len() - 1 + time as usize)
        })
    }
}

fn is_print() -> bool {
    is_flag("--print")
}

fn is_flag(flag: &str) -> bool {
    std::env::args().any(|arg| arg.as_str() == flag)
}

fn main() {
    aoc::solution!(Day24);
}

#[cfg(test)]
mod tests {
    use crate::Day24 as day_24;
    use crate::*;

    aoc::test_common!(day_24);

    aoc::test! {
        day_24:
        - "#.######\n#>>.<^<#\n#.<..<<#\n#>v.><>#\n#<^v^^>#\n######.#\n"
            => Some(18)
            => Some(54)
    }
}

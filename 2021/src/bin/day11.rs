use aoc::Solution;
use std::collections::VecDeque;

pub struct Day11;

#[derive(Debug, Clone)]
pub struct Grid {
    octopuses: Vec<u32>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_str(src: &str) -> Result<Self, &str> {
        let src = src.trim();
        let width = src.find("\n").ok_or("Invalid input")?;
        let octopuses: Vec<_> = src
            .lines()
            .flat_map(|line| line.chars().filter_map(|c| c.to_digit(10)))
            .collect();
        let height = octopuses.len() / width;

        Ok(Self {
            octopuses,
            width,
            height,
        })
    }

    fn size(&self) -> usize {
        self.width * self.height
    }

    fn flash(&mut self) -> usize {
        let mut count = 0;
        self.octopuses = self
            .octopuses
            .iter()
            .map(|&o| if o + 1 > 9 { 0 } else { o + 1 })
            .collect();

        let mut flashes: VecDeque<_> = self
            .octopuses
            .iter()
            .enumerate()
            .filter_map(|(i, o)| match &o {
                0 => Some(i),
                _ => None,
            })
            .collect();

        while let Some(pos) = flashes.pop_front() {
            count += 1;

            let p = pos as i32;
            let width = self.width as i32;
            let height = self.height as i32;

            assert_eq!((10 / width, 10 % width), (1, 0));

            [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .into_iter()
            .filter_map(|(yi, xi)| match (p / width + yi, p % width + xi) {
                (y, x) if y >= 0 && x >= 0 && y < height && x < width => {
                    Some((x + y * width) as usize)
                }
                _ => None,
            })
            .for_each(|offset| {
                self.octopuses[offset] = match self.octopuses[offset] {
                    v if v + 1 > 9 => {
                        flashes.push_back(offset);
                        0
                    }
                    v if v > 0 => v + 1,
                    0 => 0,
                    _ => panic!(),
                }
            });
        }

        count
    }
}

impl Solution<usize, usize> for Day11 {
    const DAY: u32 = 11;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Dumbo Octopus";
    type Input = Grid;

    fn part1(input: &Self::Input) -> Option<usize> {
        let (count, _) = (0..100).fold((0, input.clone()), |(mut count, mut grid), _| {
            count += grid.flash();

            (count, grid)
        });

        Some(count)
    }

    fn part2(input: &Self::Input) -> Option<usize> {
        let mut grid = input.to_owned();
        let mut step = 1;

        while grid.flash() != grid.size() {
            step += 1
        }

        Some(step)
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        Grid::from_str(&input)
    }
}

fn main() {
    Day11::run(include_str!("../../data/day11_input"));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526\n";
    #[test]
    fn day11() {
        Day11::test(INPUT, Some(1656), Some(195));
    }
}

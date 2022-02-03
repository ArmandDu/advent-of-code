use aoc::Solution;
use itertools::Itertools;

pub struct Day04;

#[derive(Copy, Clone, Debug)]
pub enum Number {
    Marked(usize),
    UnMarked(usize),
}

type Grid = Vec<Vec<Number>>;

impl Solution<usize, usize> for Day04 {
    const DAY: u32 = 4;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Giant Squid";
    type Input = Vec<usize>;

    fn part1(input: &Self::Input) -> Option<usize> {
        input.first().cloned()
    }

    fn part2(input: &Self::Input) -> Option<usize> {
        input.last().cloned()
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        let (numbers, grids) = input.trim().split_once("\n\n").expect("Invalid Input");

        let numbers = numbers
            .trim()
            .split(",")
            .map(|s| s.parse().expect("Invalid input"))
            .collect();

        let grids = grids
            .split("\n\n")
            .map(|grid| {
                grid.lines()
                    .map(|line| {
                        line.split_whitespace()
                            .map(|s| Number::UnMarked(s.parse().expect("Invalid input")))
                            .collect()
                    })
                    .collect()
            })
            .collect();

        Ok(Self::simulate(&numbers, &grids))
    }
}

impl Day04 {
    fn simulate(numbers: &Vec<usize>, grids: &Vec<Grid>) -> Vec<usize> {
        let mut grids = grids.to_owned();
        let mut solve_order = vec![];

        for number in numbers {
            for grid_index in 0..grids.len() {
                if solve_order.iter().any(|(index, _)| index == &grid_index) {
                    // grid already solved
                    continue;
                }

                (0..grids[grid_index].len())
                    .cartesian_product(0..grids[grid_index][0].len())
                    .for_each(|(y, x)| {
                        grids[grid_index][y][x] = match grids[grid_index][y][x] {
                            Number::UnMarked(value) if &value == number => Number::Marked(value),
                            o => o,
                        }
                    });

                if Self::is_winner(&grids[grid_index]) {
                    let count = Self::count_unmarked(&grids[grid_index]);

                    solve_order.push((grid_index, count * number));
                }
            }
        }

        solve_order.into_iter().map(|(_, x)| x).collect()
    }

    fn count_unmarked(grid: &Grid) -> usize {
        (0..grid.len())
            .cartesian_product(0..grid[0].len())
            .filter_map(|(y, x)| match grid[y][x] {
                Number::UnMarked(v) => Some(v),
                _ => None,
            })
            .sum()
    }

    fn is_winner(grid: &Grid) -> bool {
        Self::winning_row(grid) || Self::winning_col(grid)
    }

    fn winning_row(grid: &Grid) -> bool {
        (0..grid.len()).any(|y| grid[y].iter().all(|num| matches!(num, Number::Marked(_))))
    }
    fn winning_col(grid: &Grid) -> bool {
        (0..grid[0].len()).any(|x| (0..grid.len()).all(|y| matches!(grid[y][x], Number::Marked(_))))
    }
}

fn main() {
    Day04::run(include_str!("../../data/day04_input"));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn day04() {
        Day04::test(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7",
            Some(4512),
            Some(1924),
        )
    }
}

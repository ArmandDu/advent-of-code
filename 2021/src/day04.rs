use itertools::Itertools;
use year2021::Solution;

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

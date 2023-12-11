use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Cosmos {
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Cosmos {
    fn new(map: HashMap<(usize, usize), char>) -> Self {
        let (x, y) = map.keys().copied().max().unwrap_or_default();

        Self {
            empty_rows: (0..=x)
                .filter(|target| {
                    map.iter()
                        .filter(|((x, _), _)| x == target)
                        .map(|(_, c)| c)
                        .all_equal()
                })
                .collect(),
            empty_cols: (0..=y)
                .filter(|target| {
                    map.iter()
                        .filter(|((_, y), _)| y == target)
                        .map(|(_, c)| c)
                        .all_equal()
                })
                .collect(),
            galaxies: map
                .iter()
                .filter_map(|(coord, c)| match c {
                    '#' => Some(*coord),
                    _ => None,
                })
                .collect(),
        }
    }
}

impl FromStr for Cosmos {
    type Err = SolutionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let map = input
            .lines()
            .enumerate()
            .flat_map(|(y, row)| row.chars().enumerate().map(move |(x, c)| ((x, y), c)))
            .collect::<HashMap<_, _>>();

        Ok(Cosmos::new(map))
    }
}

impl Cosmos {
    fn galaxy_pairs(&self) -> impl Iterator<Item = (&(usize, usize), &(usize, usize))> {
        self.galaxies
            .iter()
            .combinations_with_replacement(2)
            .filter_map(|list| list.iter().cloned().collect_tuple::<(_, _)>())
            .filter(|(a, b)| a != b)
    }

    fn distance<const AGE: usize>(
        &self,
        lhs: &(usize, usize),
        rhs: &(usize, usize),
    ) -> Option<usize> {
        let (x_min, x_max) = [lhs.0, rhs.0].into_iter().minmax().into_option()?;
        let (y_min, y_max) = [lhs.1, rhs.1].into_iter().minmax().into_option()?;

        let dist = x_max - x_min + y_max - y_min;

        let cols = self
            .empty_cols
            .iter()
            .filter(|&&y| y_min < y && y < y_max)
            .count();
        let rows = self
            .empty_rows
            .iter()
            .filter(|&&x| x_min < x && x < x_max)
            .count();

        let gaps_count = (cols + rows) * 1.max(AGE - 1);

        Some(dist + gaps_count)
    }
}

struct Day11;

impl Solution for Day11 {
    const TITLE: &'static str = "Cosmic Expansion";
    const DAY: u8 = 11;
    type Input = Cosmos;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Cosmos::from_str(input)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        Some(
            input
                .galaxy_pairs()
                .filter_map(|(lhs, rhs)| input.distance::<1>(lhs, rhs))
                .sum(),
        )
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        Some(
            input
                .galaxy_pairs()
                .filter_map(|(lhs, rhs)| input.distance::<1000000>(lhs, rhs))
                .sum(),
        )
    }
}

aoc::run!(Day11);

aoc::example! {
    [Day11]
    small: "...#......\r\n.......#..\r\n#.........\r\n..........\r\n......#...\r\n.#........\r\n.........#\r\n..........\r\n.......#..\r\n#...#.....\r\n"
        => Some(374)
        => Some(82000210)
}

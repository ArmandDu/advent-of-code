use aoc::Solution;
use aoc_utils::neighbors;
use itertools::Itertools;

pub struct Day09;

impl Solution<usize, usize> for Day09 {
    const DAY: u32 = 9;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Smoke Basin";
    type Input = Vec<Vec<usize>>;

    fn part1(input: &Self::Input) -> Option<usize> {
        let low_points = Self::find_low_points(&input);

        Some(
            low_points
                .iter()
                .map(|(_, _, height)| height)
                .sum::<usize>()
                + low_points.len(),
        )
    }

    fn part2(input: &Self::Input) -> Option<usize> {
        let low_points = Self::find_low_points(&input);

        let basins = low_points
            .iter()
            .map(|&(x, y, _)| {
                let mut basin = vec![(x, y)];

                Self::add_neighbors_to_basin(&mut basin, &input, x, y);

                basin
            })
            .collect::<Vec<_>>();

        let basin_sizes = basins
            .iter()
            .map(|basin| basin.len())
            .sorted()
            .rev()
            .collect::<Vec<_>>();

        Some(basin_sizes.iter().take(3).product())
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        Ok(input
            .lines()
            .map(|line| {
                line.trim()
                    .split("")
                    .filter_map(|d| d.parse::<_>().ok())
                    .collect()
            })
            .collect())
    }
}

impl Day09 {
    fn add_neighbors_to_basin(
        basin: &mut Vec<(usize, usize)>,
        h_map: &Vec<Vec<usize>>,
        x: usize,
        y: usize,
    ) {
        neighbors(h_map[0].len(), h_map.len(), x, y)
            .into_iter()
            .filter(|&(xi, yi)| h_map[yi][xi] < 9)
            .for_each(|(xi, yi)| {
                if !basin.iter().any(|&(bx, by)| bx == xi && by == yi) {
                    basin.push((xi, yi));

                    Self::add_neighbors_to_basin(basin, &h_map, xi, yi);
                }
            });
    }

    fn find_low_points(h_map: &Vec<Vec<usize>>) -> Vec<(usize, usize, usize)> {
        let width = h_map[0].len();
        let height = h_map.len();

        (0..height)
            .cartesian_product(0..width)
            .fold(vec![], |mut low_points, (y, x)| {
                let current = h_map[y][x];

                if neighbors(h_map[0].len(), h_map.len(), x, y)
                    .into_iter()
                    .all(|(xi, yi)| h_map[yi][xi] > current)
                {
                    low_points.push((x, y, current))
                }
                low_points
            })
    }
}

fn main() {
    Day09::run(include_str!("../../data/day09_input"));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";

    #[test]
    fn day09() {
        Day09::test(INPUT, Some(15), Some(1134));
    }
}

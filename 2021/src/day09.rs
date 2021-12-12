use itertools::Itertools;
use year2021::Solution;

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
    fn neighbors(width: usize, height: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
        [
            (y > 0).then(|| (x, y - 1)),
            (x > 0).then(|| (x - 1, y)),
            (x + 1 < width).then(|| (x + 1, y)),
            (y + 1 < height).then(|| (x, y + 1)),
        ]
        .iter()
        .filter_map(|&c| c)
        .collect()
    }

    fn add_neighbors_to_basin(
        basin: &mut Vec<(usize, usize)>,
        h_map: &Vec<Vec<usize>>,
        x: usize,
        y: usize,
    ) {
        let neighbors = Self::neighbors(h_map[0].len(), h_map.len(), x, y);

        neighbors
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
                let neighbors = Self::neighbors(width, height, x, y);

                if neighbors.iter().all(|&(xi, yi)| h_map[yi][xi] > current) {
                    low_points.push((x, y, current))
                }
                low_points
            })
    }
}

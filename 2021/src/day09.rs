use itertools::Itertools;
use year2021::Solution;

pub struct Day09;

impl Solution<usize, usize> for Day09 {
    const DAY: u32 = 9;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Smoke Basin";
    type Input = (usize, usize, Vec<Vec<usize>>);

    fn part1(input: &Self::Input) -> Option<usize> {
        let (_, _, h_map) = input;
        let low_points = Self::find_low_points(&h_map);

        Some(
            low_points
                .iter()
                .map(|(_, _, height)| height)
                .sum::<usize>()
                + low_points.len(),
        )
    }

    fn part2(input: &Self::Input) -> Option<usize> {
        let (_, _, h_map) = input;
        let low_points = Self::find_low_points(&h_map);

        let basins = low_points
            .iter()
            .map(|&(x, y, _)| {
                let mut basin = vec![(x, y)];

                Self::add_neighbors_to_basin(&mut basin, &h_map, x, y);

                basin
            })
            .collect::<Vec<_>>();

        let basin_sizes = basins
            .iter()
            .map(|basin| basin.len())
            .sorted()
            .rev()
            .collect::<Vec<_>>();

        (basin_sizes.len() >= 3).then(|| basin_sizes[0] * basin_sizes[1] * basin_sizes[2])
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        let h_map: Vec<_> = input
            .lines()
            .map(|line| {
                line.trim()
                    .split("")
                    .filter_map(|d| d.parse::<_>().ok())
                    .collect()
            })
            .collect();

        let width = input.find("\n").expect("Invalid file");
        let height = h_map.len();

        Ok((width, height, h_map))
    }
}

#[derive(Debug)]
struct Pos {
    top: Option<usize>,
    left: Option<usize>,
    bottom: Option<usize>,
    right: Option<usize>,
}

impl Pos {
    pub fn new(h_map: &Vec<Vec<usize>>, x: usize, y: usize) -> Self {
        Self {
            top: (y > 0).then(|| h_map[y - 1][x]),
            left: (x > 0).then(|| h_map[y][x - 1]),
            bottom: (y + 1 < h_map.len()).then(|| h_map[y + 1][x]),
            right: (x + 1 < h_map[y].len()).then(|| h_map[y][x + 1]),
        }
    }
}

impl Day09 {
    fn add_neighbors_to_basin(
        basin: &mut Vec<(usize, usize)>,
        h_map: &Vec<Vec<usize>>,
        x: usize,
        y: usize,
    ) {
        let neighbors = Pos::new(&h_map, x, y);

        if matches!(neighbors.top, Some(val) if val < 9 && !basin.iter().any(|&(bx, by)| bx == x && by == y-1))
        {
            basin.push((x, y - 1));

            Self::add_neighbors_to_basin(basin, &h_map, x, y - 1);
        }
        if matches!(neighbors.left, Some(val) if val < 9 && !basin.iter().any(|&(bx, by)| bx == x-1 && by == y))
        {
            basin.push((x - 1, y));

            Self::add_neighbors_to_basin(basin, &h_map, x - 1, y);
        }
        if matches!(neighbors.bottom, Some(val) if val < 9 && !basin.iter().any(|&(bx, by)| bx == x && by == y+1))
        {
            basin.push((x, y + 1));

            Self::add_neighbors_to_basin(basin, &h_map, x, y + 1);
        }
        if matches!(neighbors.right, Some(val) if val < 9 && !basin.iter().any(|&(bx, by)| bx == x+1 && by == y))
        {
            basin.push((x + 1, y));

            Self::add_neighbors_to_basin(basin, &h_map, x + 1, y);
        }
    }

    fn find_low_points(h_map: &Vec<Vec<usize>>) -> Vec<(usize, usize, usize)> {
        let mut low_points = vec![];
        for y in 0..h_map.len() {
            let row = &h_map[y];

            for x in 0..row.len() {
                let current = row[x];

                let neighbors = Pos::new(&h_map, x, y);
                let is_higher_top = neighbors.top.unwrap_or(9) <= current;
                let is_higher_left = neighbors.left.unwrap_or(9) <= current;
                let is_higher_bottom = neighbors.bottom.unwrap_or(9) <= current;
                let is_higher_right = neighbors.right.unwrap_or(9) <= current;

                if !(is_higher_top || is_higher_left || is_higher_bottom || is_higher_right) {
                    low_points.push((x, y, current))
                }
            }
        }

        low_points
    }
}

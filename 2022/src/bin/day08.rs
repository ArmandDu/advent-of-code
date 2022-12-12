use aoc::Solution;

struct Day08;

impl Day08 {
    fn is_visible(input: &[Vec<u8>], x0: usize, y0: usize, dx: i8, dy: i8) -> Option<()> {
        let current = input.get(y0)?.get(x0)?;

        let mut pos = (x0, y0);

        loop {
            pos = (
                (pos.0 as i32 + dx as i32) as usize,
                (pos.1 as i32 + dy as i32) as usize,
            );
            let next = input.get(pos.1)?.get(pos.0)?;

            if next >= current {
                return Some(());
            }
        }
    }

    fn count_visible(input: &[Vec<u8>], x0: usize, y0: usize, dx: i8, dy: i8) -> usize {
        let mut count = 0;
        let mut pos = (x0, y0);
        let current = input.get(y0).unwrap().get(x0).unwrap();

        loop {
            pos = (
                (pos.0 as i32 + dx as i32) as usize,
                (pos.1 as i32 + dy as i32) as usize,
            );

            match input.get(pos.1) {
                None => return count,
                Some(row) => match row.get(pos.0) {
                    None => return count,
                    Some(next) => {
                        count += 1;
                        if next >= current {
                            return count;
                        }
                    }
                },
            }
        }
    }
}

impl Solution for Day08 {
    const TITLE: &'static str = "Treetop Tree House";
    const DAY: u8 = 8;
    type Input = Vec<Vec<u8>>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        let grid = input
            .lines()
            .map(|line| line.chars().map(|tree| tree as u8 - b'0').collect())
            .collect();

        Ok(grid)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let deltas = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        let edges_count = input.len() * 2 + input.first()?.len() * 2 - 4;
        let mut count = 0;

        for row_index in 1..(input.len() - 1) {
            let row = &input[row_index];

            for col_index in 1..(row.len() - 1) {
                if deltas.iter().any(|(dx, dy)| {
                    Day08::is_visible(input, col_index, row_index, *dx, *dy).is_none()
                }) {
                    count += 1;
                }
            }
        }

        Some(edges_count + count)
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let deltas = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        input
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().map(move |(j, _)| {
                    deltas.iter().fold(1, |acc, (dx, dy)| {
                        acc * Day08::count_visible(input, j, i, *dx, *dy)
                    })
                })
            })
            .max()
    }
}

fn main() {
    aoc::solution!(Day08)
}
#[cfg(test)]
mod tests {
    use crate::Day08 as day_08;
    use crate::*;

    aoc::test_common!(day_08);

    aoc::test! {
        day_08:
        - "30373\n25512\n65332\n33549\n35390\n"
            => Some(21)
            => Some(8)
    }
}

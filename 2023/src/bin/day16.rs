use aoc::Solution;
use aoc_utils::collections::Matrix;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct Day16;

impl Day16 {
    fn next_beam(beam: (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)> {
        Some((
            beam.0.checked_add_signed(dir.0)?,
            beam.1.checked_add_signed(dir.1)?,
        ))
    }

    fn cast(
        input: &HashMap<(usize, usize), char>,
        start: (usize, usize),
        start_dir: (isize, isize),
    ) -> HashSet<((usize, usize), (isize, isize))> {
        let mut beams = vec![];
        let mut history = HashSet::new();

        beams.push((start, start_dir));

        while let Some((beam, dir)) = beams.pop() {
            if history.contains(&(beam, dir)) {
                continue;
            }

            if let Some(tile) = input.get(&beam) {
                history.insert((beam, dir));

                let (dir_x, dir_y) = dir;

                let next_dirs = match tile {
                    '|' => {
                        if dir_y == 0 {
                            vec![(0, -1), (0, 1)]
                        } else {
                            vec![dir]
                        }
                    }
                    '-' => {
                        if dir_x == 0 {
                            vec![(1, 0), (-1, 0)]
                        } else {
                            vec![dir]
                        }
                    }
                    '/' => vec![(-dir_y, -dir_x)],
                    '\\' => {
                        vec![(dir_y, dir_x)]
                    }
                    _ => vec![dir],
                };

                for next_dir in next_dirs {
                    if let Some(next_beam) = Day16::next_beam(beam, next_dir) {
                        beams.push((next_beam, next_dir))
                    }
                }
            }
        }

        history
    }
}

impl Solution for Day16 {
    const TITLE: &'static str = "The Floor Will Be Lava";
    const DAY: u8 = 16;
    type Input = Matrix<char>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Ok(input.into())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let matrix: HashMap<_, _> = input.iter().cloned().collect();
        let history = Day16::cast(&matrix, (0, 0), (1, 0));

        Some(history.iter().unique_by(|(b, _)| b).count())
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let width = input.width();
        let height = input.height();
        let matrix: HashMap<_, _> = input.iter().cloned().collect();

        (0..width)
            .flat_map(|x| [((x, 0), (0, 1)), ((x, height - 1), (0, -1))])
            .chain((0..height).flat_map(|y| [((0, y), (1, 0)), ((width - 1, y), (-1, 0))]))
            .map(|(start, dir)| Day16::cast(&matrix, start, dir))
            .map(|hist| hist.iter().unique_by(|(b, _)| b).count())
            .max()
    }
}

aoc::run!(Day16);

aoc::example! {
    [Day16]
    simple: ".|...\\....\r\n|.-.\\.....\r\n.....|-...\r\n........|.\r\n..........\r\n.........\\\r\n..../.\\\\..\r\n.-.-/..|..\r\n.|....-|.\\\r\n..//.|....\r\n"
        => Some(46)
        => Some(51)
}

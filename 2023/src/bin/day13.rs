use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;
use std::fmt::Debug;
use std::str::FromStr;

struct Day13;

#[derive(Debug, Clone)]
struct Image(Vec<Vec<char>>);

impl Image {
    fn inner(&self) -> &[Vec<char>] {
        &self.0
    }

    fn rotate(&self) -> Self {
        let width = self.inner().first().map(|c| c.len()).unwrap_or_default();
        let height = self.inner().len();

        Self(
            (0..width)
                .map(|x| (0..height).map(|y| self.0[y][x]).collect())
                .collect(),
        )
    }

    fn find_mirror(&self, smudges: usize) -> Option<usize> {
        let image = self.inner();
        let size = image.len();

        (1..size).find(|&index| {
            let start = image[0..index].iter().rev();
            let end = &image[index..];

            start
                .zip(end)
                .map(|(l, r)| l.iter().zip(r).filter(|(l, r)| l != r).count())
                .sum::<usize>()
                == smudges
        })
    }
}

impl FromStr for Image {
    type Err = SolutionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Image(
            input.lines().map(|line| line.chars().collect()).collect(),
        ))
    }
}

impl Solution for Day13 {
    const TITLE: &'static str = "Point of Incidence";
    const DAY: u8 = 13;
    type Input = Vec<Image>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        input
            .replace('\r', "")
            .split("\n\n")
            .map(Image::from_str)
            .collect()
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        input
            .iter()
            .map(|img| {
                img.find_mirror(0).unwrap_or_default() * 100
                    + img.rotate().find_mirror(0).unwrap_or_default()
            })
            .sum1()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        input
            .iter()
            .map(|img| {
                img.find_mirror(1).unwrap_or_default() * 100
                    + img.rotate().find_mirror(1).unwrap_or_default()
            })
            .sum1()
    }
}

aoc::run!(Day13);

aoc::example! {
    [Day13]
    small: "#.##..##.\r\n..#.##.#.\r\n##......#\r\n##......#\r\n..#.##.#.\r\n..##..##.\r\n#.#.##.#.\r\n\r\n#...##..#\r\n#....#..#\r\n..##..###\r\n#####.##.\r\n#####.##.\r\n..##..###\r\n#....#..#\r\n"
        => Some(405)
        => Some(400)
}

use year2021::Solution;

use crate::utils;

pub struct Day15;

impl Solution<i32, i32> for Day15 {
    const DAY: u32 = 15;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Chiton";
    type Input = (Vec<Vec<i32>>, usize, usize);

    fn part1(input: &Self::Input) -> Option<i32> {
        let (map, width, height) = input;

        utils::a_star::solve(
            &(0, 0),
            &(*width - 1, *height - 1),
            &(*width, *height),
            |_, &(x, y)| Some(map[y][x]),
            Self::manhattan_dist,
        )
        .and_then(|(_, cost)| Some(cost))
    }

    fn part2(input: &Self::Input) -> Option<i32> {
        let (map, width, height) = input;

        let proj_width = *width * 5;
        let proj_height = *height * 5;

        utils::a_star::solve(
            &(0, 0),
            &(proj_width - 1, proj_height - 1),
            &(proj_width, proj_height),
            |_, (x, y)| {
                Some(
                    match map[y % height][x % width] + (x / width + y / height) as i32 {
                        x if x > 9 => x - 9,
                        x => x,
                    },
                )
            },
            Self::manhattan_dist,
        )
        .and_then(|(_, total_cost)| Some(total_cost))
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        let map: Vec<Vec<i32>> = input
            .trim()
            .lines()
            .map(|line| line.chars().map(|c| (c as u8 - b'0') as i32).collect())
            .collect();
        let height = map.len();
        let width = input.find("\n").expect("Invalid input");

        Ok((map, width, height))
    }
}

impl Day15 {
    pub fn manhattan_dist(a: &(usize, usize), b: &(usize, usize)) -> i32 {
        let (ax, ay) = *a;
        let (bx, by) = *b;

        (ax as i32 - bx as i32).abs() + (ay as i32 - by as i32).abs()
    }
}

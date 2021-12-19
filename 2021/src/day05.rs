use std::cmp::Ordering;
use std::collections::HashMap;

use year2021::Solution;

pub struct Day05;

impl Solution<usize, usize> for Day05 {
    const DAY: u32 = 5;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Hydrothermal Venture";
    type Input = Vec<((i32, i32), (i32, i32))>;

    fn part1(input: &Self::Input) -> Option<usize> {
        let canvas = input.iter().fold(HashMap::new(), |mut canvas, line| {
            match line {
                (a, b) if a.0 == b.0 || a.1 == b.1 => Self::draw_line(&mut canvas, &a, &b),
                _ => {}
            }

            canvas
        });

        Some(canvas.values().filter(|&&value| value > 1).count())
    }

    fn part2(input: &Self::Input) -> Option<usize> {
        let canvas = input.iter().fold(HashMap::new(), |mut canvas, (a, b)| {
            Self::draw_line(&mut canvas, &a, &b);

            canvas
        });

        Some(canvas.values().filter(|&&value| value > 1).count())
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        Ok(input
            .trim()
            .lines()
            .map(|line| {
                let (a, b) = line.split_once(" -> ").expect("Invalid input");
                let (xa, ya) = a.split_once(",").expect("Invalid input");
                let (xb, yb) = b.split_once(",").expect("Invalid input");

                (
                    (xa.parse().unwrap(), ya.parse().unwrap()),
                    (xb.parse().unwrap(), yb.parse().unwrap()),
                )
            })
            .collect())
    }
}

impl Day05 {
    pub fn draw_line(canvas: &mut HashMap<(i32, i32), i32>, start: &(i32, i32), end: &(i32, i32)) {
        let (mut x0, mut y0) = start.clone();

        let dx = match x0.cmp(&end.0) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        let dy = match y0.cmp(&end.1) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        *canvas.entry(*end).or_insert(0) += 1;

        while (x0, y0) != *end {
            *canvas.entry((x0, y0)).or_insert(0) += 1;

            x0 += dx;
            y0 += dy;
        }
    }
}

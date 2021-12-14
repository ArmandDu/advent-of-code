use itertools::Itertools;
use std::collections::HashSet;
use year2021::Solution;

pub struct Day13;
pub type DotSet = HashSet<(i32, i32)>;

impl Solution<usize, String> for Day13 {
    const DAY: u32 = 13;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Transparent Origami";
    type Input = (DotSet, Vec<(i32, i32)>);

    fn part1(input: &Self::Input) -> Option<usize> {
        let (dots, instructions) = input;

        Some(
            instructions
                .iter()
                .take(1)
                .fold(dots.clone(), Self::fold)
                .len(),
        )
    }

    fn part2(input: &Self::Input) -> Option<String> {
        let (dots, instructions) = input;

        let code = instructions.iter().fold(dots.clone(), Self::fold);
        let width = code.iter().map(|(x, _)| x).max().unwrap() + 1;
        let height = code.iter().map(|(_, y)| y).max().unwrap() + 1;

        let sheet = (0..height)
            .map(|y| {
                (0..width).fold(String::new(), |acc, x| {
                    acc + code.get(&(x, y)).and(Some("▓")).or(Some(" ")).unwrap()
                })
            })
            .join("\n");

        Some("\n".to_string() + &sheet)
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        let (dots, instructions) = input.split_once("\n\n").expect("Invalid input");

        let dots: DotSet = dots
            .lines()
            .map(|coord| match coord.split_once(",") {
                Some((x, y)) => (x.parse::<i32>().unwrap(), y.parse::<_>().unwrap()),
                _ => panic!(),
            })
            .collect();

        let instructions: Vec<_> = instructions
            .lines()
            .map(|line| match line.split_once("=") {
                Some(("fold along y", val)) => (0, val.parse::<i32>().unwrap()),
                Some(("fold along x", val)) => (val.parse::<i32>().unwrap(), 0),
                _ => panic!(),
            })
            .collect();
        Ok((dots, instructions))
    }
}

impl Day13 {
    fn fold(dots: DotSet, pivot: &(i32, i32)) -> DotSet {
        let sheet_height = dots.iter().map(|(_, y)| y).max().unwrap();
        let new_height = pivot.1.max(sheet_height - pivot.1);

        let sheet_width = dots.iter().map(|(x, _)| x).max().unwrap();
        let new_width = pivot.0.max(sheet_width - pivot.0);

        dots.into_iter()
            .map(|(x, y)| {
                (
                    match x {
                        coord if coord > new_width => new_width - (coord - new_width).abs(),
                        coord => coord,
                    },
                    match y {
                        coord if coord > new_height => new_height - (coord - new_height).abs(),
                        coord => coord,
                    },
                )
            })
            .collect()
    }
}

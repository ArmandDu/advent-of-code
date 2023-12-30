use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;
use regex::Regex;
use std::str::FromStr;

struct Day03;

type Number = ((usize, usize), String);
type Symbol = ((usize, usize), char);

#[derive(Debug)]
struct Schematic {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

impl Schematic {
    fn in_range(num: &Number, sym: &Symbol) -> bool {
        let ((sx, sy), _) = *sym;
        let ((xn0, yn), n) = num;
        let xn1 = xn0 + n.len();

        [
            (sy > 0).then_some((sx, sy - 1)),
            (sy > 0).then_some((sx + 1, sy - 1)),
            (sx > 0).then_some((sx - 1, sy)),
            (sx > 0).then_some((sx - 1, sy + 1)),
            (sx > 0 && sy > 0).then_some((sx - 1, sy - 1)),
            Some((sx + 1, sy)),
            Some((sx + 1, sy + 1)),
            Some((sx, sy + 1)),
        ]
        .iter()
        .filter_map(|&c| c)
        .any(|(dx, dy)| dy == *yn && dx >= *xn0 && dx < xn1)
    }

    fn get_parts(&self, symbol: &Symbol) -> Vec<u32> {
        self.numbers
            .iter()
            .filter(|number| Schematic::in_range(number, symbol))
            .map(|(_, number)| number.parse::<u32>().unwrap())
            .unique()
            .collect_vec()
    }
}

impl FromStr for Schematic {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+)+").map_err(|_| SolutionError::ParseError)?;

        let base = Schematic {
            symbols: vec![],
            numbers: vec![],
        };

        let result = s.lines().enumerate().fold(base, |mut schema, (y, line)| {
            re.find_iter(line).for_each(|number| {
                schema
                    .numbers
                    .push(((number.start(), y), number.as_str().into()))
            });

            line.char_indices().for_each(|(x, c)| match c {
                c if !c.is_numeric() && c != '.' => schema.symbols.push(((x, y), c)),
                _ => {}
            });

            schema
        });

        Ok(result)
    }
}

impl Solution for Day03 {
    const TITLE: &'static str = "Gear Ratios";
    const DAY: u8 = 3;
    type Input = Schematic;
    type P1 = u32;
    type P2 = u32;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        input.parse()
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        input
            .symbols
            .iter()
            .flat_map(|symbol| input.get_parts(symbol))
            .sum1()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        input
            .symbols
            .iter()
            .filter(|(_, sym)| sym == &'*')
            .map(|symbol| {
                let parts = input.get_parts(symbol);

                match parts.len() {
                    2 => parts.iter().product::<u32>(),
                    _ => 0,
                }
            })
            .sum1()
    }
}

fn main() {
    aoc::solution!(Day03)
}
#[cfg(test)]
mod tests {
    use crate::Day03 as day_03;
    use crate::*;

    aoc::test_common!(day_03);

    aoc::test! {
        day_03:
        [example]
        - "467..114..\r\n...*......\r\n..35..633.\r\n......#...\r\n617*......\r\n.....+.58.\r\n..592.....\r\n......755.\r\n...$.*....\r\n.664.598..\r\n"
            => Some(4361)
            => Some(467835);

    }
}

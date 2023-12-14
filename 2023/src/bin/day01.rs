use aoc::Solution;
use itertools::Itertools;

macro_rules! s {
    ($( $e:expr)+ ) => {
        [$( stringify!($e), )+].iter().enumerate().map(|(i, c)| (c.to_owned(), i % 10)).collect()
    }
}

struct Day01;

impl Day01 {
    fn list_to_number(list: &[usize]) -> Option<usize> {
        let first = list.first().map(|&x| x * 10)?;

        list.last().map(|&last| first + last)
    }

    fn solve(input: &str, mapping: &[(&str, usize)]) -> Option<usize> {
        let (_, first) = (0..input.len())
            .map(|offset| &input[offset..])
            .find_map(|input| mapping.iter().find(|(key, _)| input.starts_with(key)))?;

        let (_, last) = (0..=input.len())
            .rev()
            .map(|offset| &input[0..offset])
            .find_map(|input| mapping.iter().find(|(key, _)| input.ends_with(key)))?;

        Some(first * 10 + last)
    }
}

impl Solution for Day01 {
    const TITLE: &'static str = "Trebuchet?!";
    const DAY: u8 = 1;
    type Input = Vec<String>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Ok(input.lines().map(|line| line.to_owned()).collect())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        input
            .iter()
            .map(|input| {
                input
                    .chars()
                    .filter_map(|c| c.to_digit(10).map(|n| n as usize))
                    .collect_vec()
            })
            .filter_map(|input| Day01::list_to_number(&input))
            .sum1()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let mapping: Vec<_> =
            s! { zero one two three four five six seven eight nine 0 1 2 3 4 5 6 7 8 9 };

        input
            .iter()
            .filter_map(|input| Day01::solve(input, &mapping))
            .sum1()
    }
}

fn main() {
    aoc::solution!(Day01)
}
#[cfg(test)]
mod tests {
    use crate::Day01 as day_01;
    use crate::*;

    aoc::test_common!(day_01);

    aoc::test! {
        day_01:
        [digits]
        - "1abc2\r\npqr3stu8vwx\r\na1b2c3d4e5f\r\ntreb7uchet\r\n"
            => Some(12+38+15+77)
            => Some(12+38+15+77);
        [spelled_out]
        - "two1nine\r\neightwothree\r\nabcone2threexyz\r\nxtwone3four\r\n4nineeightseven2\r\nzoneight234\r\n7pqrstsixteen\r\n"
            => Some(11+22+33+42+24+77)
            => Some(29+83+13+24+42+14+76);
    }
}

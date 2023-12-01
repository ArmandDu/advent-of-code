use aoc::Solution;
use itertools::Itertools;
use regex::Regex;

enum Code {
    Digit(u32),
    Word(u32),
}

impl TryFrom<&str> for Code {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "one" => Ok(Code::Word(1)),
            "two" => Ok(Code::Word(2)),
            "three" => Ok(Code::Word(3)),
            "four" => Ok(Code::Word(4)),
            "five" => Ok(Code::Word(5)),
            "six" => Ok(Code::Word(6)),
            "seven" => Ok(Code::Word(7)),
            "eight" => Ok(Code::Word(8)),
            "nine" => Ok(Code::Word(9)),
            token => token.parse().map(Code::Digit).map_err(|_| ()),
        }
    }
}

struct Day01;

impl Day01 {
    fn list_to_number(list: Vec<&u32>) -> Option<u32> {
        let first = list.first().map(|&x| x * 10)?;

        list.last().map(|&last| first + last)
    }
}

impl Solution for Day01 {
    const TITLE: &'static str = "Trebuchet?!";
    const DAY: u8 = 1;
    type Input = Vec<Vec<Code>>;
    type P1 = u32;
    type P2 = u32;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        let re = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();

        let list = input
            .lines()
            .map(|line| {
                re.captures_iter(line)
                    .filter_map(|captures| Code::try_from(captures.extract::<1>().0).ok())
                    .collect_vec()
            })
            .collect();

        Ok(list)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        Some(
            input
                .iter()
                .map(|list| {
                    list.iter()
                        .filter_map(|code| match code {
                            Code::Digit(x) => Some(x),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
                .filter_map(Day01::list_to_number)
                .sum(),
        )
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        Some(
            input
                .iter()
                .map(|list| {
                    list.iter()
                        .map(|code| match code {
                            Code::Digit(x) => x,
                            Code::Word(x) => x,
                        })
                        .collect_vec()
                })
                .filter_map(Day01::list_to_number)
                .sum(),
        )
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

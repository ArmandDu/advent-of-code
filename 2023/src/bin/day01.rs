use aoc::Solution;
use itertools::Itertools;
use regex::Regex;

struct Day01;

impl Solution for Day01 {
    const TITLE: &'static str = "Trebuchet?!";
    const DAY: u8 = 1;
    type Input = Vec<String>;
    type P1 = u32;
    type P2 = u32;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        let list = input.lines().map(|line| line.to_owned()).collect();

        Ok(list)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        Some(
            input
                .iter()
                .map(|line| {
                    line.chars()
                        .filter_map(|line| line.to_digit(10))
                        .collect::<Vec<_>>()
                })
                .filter_map(|line| {
                    let first = line.first()?.to_owned();
                    let last = line.last()?.to_owned();

                    Some(first * 10 + last)
                })
                .sum(),
        )
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let re = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();

        Some(
            input
                .iter()
                .map(|line| {
                    re.captures_iter(line)
                        .filter_map(|c| match c.extract() {
                            (_, [x]) if x == "1" || x == "one" => Some(1),
                            (_, [x]) if x == "2" || x == "two" => Some(2),
                            (_, [x]) if x == "3" || x == "three" => Some(3),
                            (_, [x]) if x == "4" || x == "four" => Some(4),
                            (_, [x]) if x == "5" || x == "five" => Some(5),
                            (_, [x]) if x == "6" || x == "six" => Some(6),
                            (_, [x]) if x == "7" || x == "seven" => Some(7),
                            (_, [x]) if x == "8" || x == "eight" => Some(8),
                            (_, [x]) if x == "9" || x == "nine" => Some(9),
                            (_, _) => None,
                        })
                        .collect_vec()
                })
                .filter_map(|line| {
                    let first = line.first()?.to_owned();
                    let last = line.last()?.to_owned();

                    Some(first * 10 + last)
                })
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

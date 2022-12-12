use aoc::Solution;
use itertools::Itertools;

struct Day06;

impl Day06 {
    fn get_marker(input: &str, size: usize) -> Option<usize> {
        input
            .chars()
            .collect_vec()
            .windows(size)
            .find_position(|window| {
                let mut slice = window.to_vec();
                slice.sort();
                slice.dedup();

                slice.len() == window.len()
            })
            .map(|(pos, _)| pos + size)
    }
}

impl Solution for Day06 {
    const TITLE: &'static str = "Tuning Trouble";
    const DAY: u8 = 6;
    type Input = String;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Ok(input.to_owned())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        Day06::get_marker(input, 4)
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        Day06::get_marker(input, 14)
    }
}

fn main() {
    aoc::solution!(Day06)
}
#[cfg(test)]
mod tests {
    use crate::Day06 as day_06;
    use crate::*;

    aoc::test_common!(day_06);

    aoc::test! {
     day_06:
     [case_1]
        - "mjqjpqmgbljsphdztnvjfqwrcgsmlb" => Some(7) => Some(19);
     [case_2]
        - "bvwbjplbgvbhsrlpgdmjqwftvncz" => Some(5) => Some(23);
     [case_3]
        - "nppdvjthqldpwncqszvftbrmjlhg" => Some(6) => Some(23);
     [case_4]
        - "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => Some(10) => Some(29);
     [case_5]
        - "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => Some(11) => Some(26);
    }
}

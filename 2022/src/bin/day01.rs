use aoc::Solution;

struct Day01;

impl Solution for Day01 {
    const TITLE: &'static str = "Calorie Counting";
    const DAY: u8 = 1;
    type Input = Vec<usize>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        let list = input
            .split("\r\n\r\n")
            .map(|elf| {
                elf.lines()
                    .filter_map(|line| line.parse::<usize>().ok())
                    .sum()
            })
            .collect();

        Ok(list)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        input.iter().max().cloned()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let mut input: Vec<usize> = input.clone();

        input.sort_by(|a, b| b.cmp(a));

        Some(input.iter().take(3).sum())
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
        - "1000\r\n2000\r\n3000\r\n\r\n4000\r\n\r\n5000\r\n6000\r\n\r\n7000\r\n8000\r\n9000\r\n\r\n10000"
            => Some(24000)
            => Some(45000)
    }
}

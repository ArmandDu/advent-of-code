use aoc::Solution;

struct Day04;

impl Solution for Day04 {
    const TITLE: &'static str = "Camp Cleanup";
    const DAY: u8 = 4;
    type Input = Vec<((usize, usize), (usize, usize))>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        let parse = |x: &str| {
            x.split_once('-')
                .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        };

        Ok(input
            .lines()
            .filter_map(|line| {
                let (left, right) = line.split_once(',')?;

                let left = parse(left)?;
                let right = parse(right)?;

                Some((left, right))
            })
            .collect())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let clamp =
            |a: &(usize, usize), b: &(usize, usize)| (a.0.clamp(b.0, b.1), a.1.clamp(b.0, b.1));

        Some(
            input
                .iter()
                .filter(|(left, right)| clamp(left, right).eq(left) || clamp(right, left).eq(right))
                .count(),
        )
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        Some(
            input
                .iter()
                .filter(|(left, right)| {
                    let mut left = (left.0)..=(left.1);
                    let right = (right.0)..=(right.1);

                    left.any(|x| right.contains(&x))
                })
                .count(),
        )
    }
}

fn main() {
    aoc::solution!(Day04)
}
#[cfg(test)]
mod tests {
    use crate::Day04 as day_04;
    use crate::*;

    aoc::test_common!(day_04);

    aoc::test! {
        day_04:
        - "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"
            => Some(2)
            => Some(4)
    }
}

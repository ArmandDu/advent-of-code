use aoc::Solution;

struct Day20;

impl Day20 {
    fn mix(initial: Vec<(usize, i64)>, order: &[(usize, i64)]) -> Vec<(usize, i64)> {
        let len = initial.len() as i64;

        order.iter().fold(initial, |mut buffer, target| {
            if let Some(index) = buffer.iter().position(|value| value == target) {
                let (origin, value) = buffer.remove(index);
                let index = Self::index(index, value, len - 1);

                buffer.insert(index as usize, (origin, value));
            }

            buffer
        })
    }

    fn index(x: usize, offset: i64, max: i64) -> usize {
        ((max + (x as i64 + (offset % max))) % max) as usize
    }
}

impl Solution for Day20 {
    const TITLE: &'static str = "Grove Positioning System";
    const DAY: u8 = 20;
    type Input = Vec<(usize, i64)>;
    type P1 = i64;
    type P2 = i64;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Ok(input
            .lines()
            .filter_map(|line| line.parse().ok())
            .enumerate()
            .collect())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let mixed = Day20::mix(input.to_owned(), input);
        let p0 = mixed.iter().position(|(_, x)| x == &0)?;
        let len = mixed.len() as i64;

        let (_, a) = mixed.get(Day20::index(p0, 1000, len))?;
        let (_, b) = mixed.get(Day20::index(p0, 2000, len))?;
        let (_, c) = mixed.get(Day20::index(p0, 3000, len))?;

        Some((a + b + c) as i64)
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        const DECRYPT_KEY: i64 = 811589153;
        let initial: Vec<_> = input
            .clone()
            .into_iter()
            .map(|(i, x)| (i, x * DECRYPT_KEY))
            .collect();

        let mixed = (0..10).fold(initial.to_owned(), |buffer, _i| {
            Day20::mix(buffer, &initial)
        });
        let p0 = mixed.iter().position(|(_, x)| x == &0)?;
        let len = mixed.len() as i64;

        let (_, a) = mixed.get(Day20::index(p0, 1000, len))?;
        let (_, b) = mixed.get(Day20::index(p0, 2000, len))?;
        let (_, c) = mixed.get(Day20::index(p0, 3000, len))?;

        Some((a + b + c) as i64)
    }
}

fn main() {
    aoc::solution!(Day20)
}
#[cfg(test)]
mod tests {
    use crate::Day20 as day_20;
    use crate::*;

    aoc::test_common!(day_20);

    aoc::test! {
        day_20:
        - "1\n2\n-3\n3\n-2\n0\n4\n"
            => Some(3)
            => Some(1623178306)
    }
}

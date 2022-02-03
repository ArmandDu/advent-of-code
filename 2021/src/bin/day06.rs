use aoc::Solution;

pub struct Day06;

impl Solution<usize, usize> for Day06 {
    const DAY: u32 = 6;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Lanternfish";
    type Input = [usize; 9];

    fn part1(input: &Self::Input) -> Option<usize> {
        Some(Self::making_babies(&input[..], 80))
    }

    fn part2(input: &Self::Input) -> Option<usize> {
        Some(Self::making_babies(&input[..], 256))
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        Ok(input
            .trim()
            .split(",")
            .filter_map(|i| i.parse::<usize>().ok())
            .fold([0; 9], |mut map, i| {
                map[i] += 1;
                map
            }))
    }
}

impl Day06 {
    fn making_babies(pool: &[usize], days: usize) -> usize {
        (0..days)
            .fold(pool.to_vec(), |mut pool, _day| {
                let new_babies = pool[0];

                pool.rotate_left(1);
                pool[6] += new_babies;

                pool
            })
            .iter()
            .sum()
    }
}

pub struct Day06V0;

impl Solution<usize, usize> for Day06V0 {
    const DAY: u32 = 6;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Lanternfish (naive version)";
    type Input = Vec<usize>;

    fn part1(input: &Self::Input) -> Option<usize> {
        Some(Self::get_population(&input, 80).len())
    }

    fn part2(_input: &Self::Input) -> Option<usize> {
        None

        // This would take Ages :D
        // Some(Self::get_population(&input, 256).len())
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        Ok(input
            .trim()
            .split(",")
            .map(|i| i.parse::<usize>().unwrap())
            .collect())
    }
}

impl Day06V0 {
    fn get_population(initial: &[usize], target: usize) -> Vec<usize> {
        (0..target).fold(initial.into(), |pool, _day| {
            let mut newborns: Vec<usize> = vec![];

            let mut pool: Vec<usize> = pool
                .iter()
                .map(|&life| {
                    if life == 0 {
                        newborns.push(8);
                        6
                    } else {
                        life - 1
                    }
                })
                .collect();

            pool.append(&mut newborns);
            pool
        })
    }
}

fn main() {
    Day06V0::run(include_str!("../../data/day06_input"));
    Day06::run(include_str!("../../data/day06_input"));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "3,4,3,1,2";

    #[test]
    fn day06_v0() {
        Day06V0::test(INPUT, Some(5934), Some(26984457539));
    }
    #[test]
    fn day06() {
        Day06::test(INPUT, Some(5934), Some(26984457539));
    }
}

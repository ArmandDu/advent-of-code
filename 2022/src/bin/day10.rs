use aoc::Solution;
use itertools::Itertools;

struct Day10;

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(i32),
}

impl Solution for Day10 {
    const TITLE: &'static str = "Cathode-Ray Tube";
    const DAY: u8 = 10;
    type Input = Vec<Instruction>;
    type P1 = i32;
    type P2 = String;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Ok(input
            .lines()
            .filter_map(|line| {
                let mut op = line.split(' ');

                Some(match op.next() {
                    Some("noop") => vec![Instruction::Noop],
                    Some("addx") => vec![
                        Instruction::Noop,
                        Instruction::Add(op.next()?.parse().ok()?),
                    ],
                    _ => unreachable!(),
                })
            })
            .flatten()
            .collect())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let (_, sum) = input.iter().take(220).zip(1..).fold(
            (1, 0),
            |(mut x, mut sum), (instruction, cycle)| {
                if [20, 60, 100, 140, 180, 220].contains(&(cycle)) {
                    sum += cycle * x;
                }

                match instruction {
                    Instruction::Noop => {}
                    Instruction::Add(y) => x += y,
                };

                (x, sum)
            },
        );

        Some(sum)
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let (output, _) = input.iter().zip(0..).fold(
            (String::new(), 1),
            |(mut output, mut x), (instruction, cycle)| {
                match [x - 1, x, x + 1].contains(&(cycle % 40)) {
                    true => output.push('#'),
                    false => output.push(' '),
                };

                match instruction {
                    Instruction::Noop => {}
                    Instruction::Add(y) => x += y,
                };

                (output, x)
            },
        );
        Some(format!(
            "\n{}",
            output
                .chars()
                .chunks(40)
                .into_iter()
                .map(|c| c.collect::<String>())
                .join("\n")
        ))
    }
}

fn main() {
    aoc::solution!(Day10)
}
#[cfg(test)]
mod tests {
    use crate::Day10 as day_10;
    use crate::*;

    aoc::test_common!(day_10);

    aoc::test! {
        day_10:
        - "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop\n"
            => Some(13140)
            => Some("\n##  ##  ##  ##  ##  ##  ##  ##  ##  ##  \n###   ###   ###   ###   ###   ###   ### \n####    ####    ####    ####    ####    \n#####     #####     #####     #####     \n######      ######      ######      ####\n#######       #######       #######     ".to_owned())
    }
}

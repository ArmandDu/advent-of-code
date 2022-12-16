use aoc::solution::SolutionError;
use aoc::Solution;
use std::collections::VecDeque;

struct Day05;

impl Solution for Day05 {
    const TITLE: &'static str = "Supply Stacks";
    const DAY: u8 = 5;
    type Input = (Vec<VecDeque<String>>, Vec<(usize, usize, usize)>);
    type P1 = String;
    type P2 = String;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        //skip first character as it's manually added for fixing the "trim" issue.
        let (stacks, instructions) = &input[1..]
            .split_once("\n\n")
            .ok_or(SolutionError::ParseError)?;

        let transposed_stacks = {
            let mut stacks = stacks
                .lines()
                .map(|line| {
                    line.chars()
                        .collect::<Vec<_>>()
                        .chunks(4)
                        .map(|chunk| chunk.get(1).cloned().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            stacks.pop();
            stacks
        };

        let mut stacks: Vec<VecDeque<String>> = vec![];

        transposed_stacks.iter().for_each(|row| {
            row.iter().enumerate().for_each(|(j, value)| {
                if stacks.get(j).is_none() {
                    stacks.push(VecDeque::new());
                }

                if value == &' ' {
                    return;
                }
                stacks.get_mut(j).unwrap().push_front(value.to_string());
            });
        });

        let instructions: Vec<_> = instructions
            .lines()
            .map(|line| {
                let words: Vec<_> = line
                    .split(' ')
                    .filter_map(|word| word.parse::<usize>().ok())
                    .collect();

                (words[0], words[1], words[2])
            })
            .collect();

        Ok((stacks, instructions))
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let (stacks, instructions) = input;
        let mut stacks = stacks.to_owned();

        for (count, from, to) in instructions {
            for _ in 0..*count {
                let from = stacks.get_mut(from - 1)?.pop_back()?;

                stacks.get_mut(to - 1)?.push_back(from);
            }
        }

        Some(
            stacks
                .iter()
                .filter_map(|stack| stack.back())
                .cloned()
                .collect::<String>(),
        )
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let (stacks, instructions) = input;
        let mut stacks = stacks.to_owned();

        for (count, from, to) in instructions {
            let mut buffer = vec![];

            for _ in 0..*count {
                let from = stacks.get_mut(from - 1)?.pop_back()?;

                buffer.push(from);
            }

            buffer.into_iter().rev().for_each(|value| {
                stacks.get_mut(to - 1).unwrap().push_back(value);
            })
        }

        Some(
            stacks
                .iter()
                .filter_map(|stack| stack.back())
                .cloned()
                .collect::<String>(),
        )
    }

    fn get_input() -> aoc::solution::Result<String> {
        let path = format!("inputs/DAY_{:02}.txt", Self::DAY);
        let input = std::fs::read_to_string(&path)?;

        //my aoc-runner library trims the input automatically. In this exercise it becomes a bug.
        //so I add a token here to void the trim
        Ok(format!("!{}", input))
    }
}

fn main() {
    aoc::solution!(Day05)
}
#[cfg(test)]
mod tests {
    use crate::Day05 as day_05;
    use crate::*;

    aoc::test_common!(day_05);

    aoc::test! {
        day_05:
        - "!    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n"
            => Some("CMZ".to_owned())
            => Some("MCD".to_owned())
    }
}

use aoc::Solution;
use itertools::Itertools;

pub struct Day08;

impl Solution<usize, usize> for Day08 {
    const DAY: u32 = 8;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Seven Segment Search";
    type Input = Vec<(Vec<String>, Vec<String>)>;

    fn part1(input: &Self::Input) -> Option<usize> {
        let count_1_4_7_or_8: usize = input
            .iter()
            .map(|(_, output)| {
                output
                    .iter()
                    .filter(|&wires| matches!(wires.len(), 2 | 4 | 3 | 7))
                    .count()
            })
            .sum();

        Some(count_1_4_7_or_8)
    }

    fn part2(input: &Self::Input) -> Option<usize> {
        Some(
            input
                .iter()
                .map(|(unique, output)| {
                    let known_wires = Self::get_known_wires(unique);

                    output.iter().fold(0_usize, |value, wires| {
                        (value * 10) + Self::get_value_from_known_wires(&wires, &known_wires)
                    })
                })
                .sum::<usize>(),
        )
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        Ok(input
            .lines()
            .map(|line| {
                let entries: Vec<_> = line.split_whitespace().map(|l| l.to_string()).collect();
                let input = entries[0..10].to_owned();
                let output = entries[11..15].to_owned();

                (input, output)
            })
            .collect())
    }
}

impl Day08 {
    fn get_known_wires(input: &Vec<String>) -> Vec<String> {
        [2, 4, 3, 7]
            .into_iter()
            .map(|size| {
                input
                    .iter()
                    .find(|&wires| wires.len() == size)
                    .unwrap()
                    .to_string()
            })
            .collect()
    }

    fn count_shared_wires(wires: &String, known_wires: &Vec<String>) -> Vec<usize> {
        known_wires
            .iter()
            .map(|known_wires| {
                known_wires
                    .chars()
                    .filter(|wire| wires.contains(&wire.to_string()))
                    .count()
            })
            .collect()
    }

    fn get_value_from_known_wires(wires: &String, known_wires: &Vec<String>) -> usize {
        let (shared_one_wires_count, shared_four_wires_count, _, _) =
            Self::count_shared_wires(&wires, &known_wires)
                .into_iter()
                .collect_tuple::<(_, _, _, _)>()
                .unwrap();

        // two first digits were enough, no need to compare with 7s or 8s
        match (wires.len(), shared_one_wires_count, shared_four_wires_count) {
            (6, 2, 3) => 0,
            (2, _, _) => 1,
            (5, 1, 2) => 2,
            (5, 2, 3) => 3,
            (4, _, _) => 4,
            (5, 1, 3) => 5,
            (6, 1, 3) => 6,
            (3, _, _) => 7,
            (7, _, _) => 8,
            (6, 2, 4) => 9,
            _ => 0,
        }
    }
}

fn main() {
    Day08::run(include_str!("../../data/day08_input"));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn day8() {
        Day08::test(INPUT, Some(26), Some(61229));
    }
}

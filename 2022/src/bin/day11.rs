use aoc::Solution;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Mul(usize),
    Add(usize),
    MulSelf,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    test: usize,
    throws: (usize, usize),
}

impl Monkey {
    fn extract<T: FromStr>(input: &str, prefix: &str) -> Option<T> {
        input.trim().strip_prefix(prefix)?.parse().ok()
    }

    fn extract_list<T: FromStr>(input: &str, prefix: &str) -> Option<Vec<T>> {
        Some(
            input
                .trim()
                .strip_prefix(prefix)?
                .split(", ")
                .filter_map(|item| item.parse::<T>().ok())
                .collect(),
        )
    }

    fn extract_operation(input: &str, prefix: &str) -> Option<Operation> {
        let operation = input.trim().strip_prefix(prefix)?;

        match operation.split_at("old * ".len()) {
            ("old * ", "old") => Some(Operation::MulSelf),
            ("old * ", n) => Some(Operation::Mul(n.parse().ok()?)),
            ("old + ", n) => Some(Operation::Add(n.parse().ok()?)),
            _ => unreachable!(),
        }
    }
}

struct Day11;

impl Day11 {
    fn rounds<const SIZE: usize>(
        monkeys: &[Monkey],
        worry: impl Fn(usize) -> usize,
    ) -> Vec<(Monkey, usize)> {
        let initial = monkeys.iter().map(|m| (m.to_owned(), 0)).collect_vec();

        (0..SIZE).fold(initial, |mut payload, _| {
            for index in 0..payload.len() {
                let (monkey, count) = &mut payload[index];

                let (if_true, if_false) = monkey.throws;
                let cond = monkey.test;

                let items = monkey
                    .items
                    .drain(..)
                    .map(|item| match monkey.op {
                        Operation::Mul(n) => item * n,
                        Operation::Add(n) => item + n,
                        Operation::MulSelf => item * item,
                    })
                    .map(&worry)
                    .collect_vec();

                *count += items.len();

                items.into_iter().for_each(|item| {
                    match item % cond {
                        0 => payload[if_true].0.items.push(item),
                        _ => payload[if_false].0.items.push(item),
                    };
                })
            }

            payload
        })
    }
}

impl Solution for Day11 {
    const TITLE: &'static str = "Monkey in the Middle";
    const DAY: u8 = 11;
    type Input = Vec<Monkey>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Ok(input
            .lines()
            .filter(|line| !line.is_empty())
            .tuples::<(&str, &str, &str, &str, &str, &str)>()
            .filter_map(|line| {
                let (_, items, operation, test, is_true, is_false) = line;

                Some(Monkey {
                    items: Monkey::extract_list(items, "Starting items: ")?,
                    op: Monkey::extract_operation(operation, "Operation: new = ")?,
                    test: Monkey::extract(test, "Test: divisible by ")?,
                    throws: (
                        Monkey::extract(is_true, "If true: throw to monkey ")?,
                        Monkey::extract(is_false, "If false: throw to monkey ")?,
                    ),
                })
            })
            .collect())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let result = Day11::rounds::<20>(input, |item| item / 3);

        Some(
            result
                .iter()
                .map(|(_, count)| count)
                .sorted()
                .skip(result.len() - 2)
                .product(),
        )
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let tests_products: usize = input.iter().map(|monkey| monkey.test).product();
        let result = Day11::rounds::<10000>(input, |item| item % tests_products);

        Some(
            result
                .iter()
                .map(|(_, count)| count)
                .sorted()
                .skip(result.len() - 2)
                .product(),
        )
    }
}

fn main() {
    aoc::solution!(Day11)
}
#[cfg(test)]
mod tests {
    use crate::Day11 as day_11;
    use crate::*;

    aoc::test_common!(day_11);

    aoc::test! {
     day_11:
     [case_1]
        - "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1\n"
            => Some(10605)
            => Some(2713310158);
    }
}

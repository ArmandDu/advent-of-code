use aoc::Solution;
use itertools::Itertools;
use serde_json::{from_str, json, Value};
use std::cmp::Ordering;

struct Day13;

impl Day13 {
    fn cmp(left: &Value, right: &Value) -> Ordering {
        match (left, right) {
            (Value::Array(left), Value::Array(right)) => left
                .iter()
                .zip(right.iter())
                .find_map(
                    |(left_value, right_value)| match Day13::cmp(left_value, right_value) {
                        Ordering::Equal => None,
                        ord => Some(ord),
                    },
                )
                .unwrap_or_else(|| left.len().cmp(&right.len())),
            (Value::Number(left), Value::Number(right)) => left.as_i64().cmp(&right.as_i64()),
            (Value::Number(left), Value::Array(right)) => Day13::cmp(&json!([left]), &json!(right)),
            (Value::Array(left), Value::Number(right)) => Day13::cmp(&json!(left), &json!([right])),
            _ => unreachable!(),
        }
    }
}

impl Solution for Day13 {
    const TITLE: &'static str = "Distress Signal";
    const DAY: u8 = 13;
    type Input = Vec<Value>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Ok(input
            .lines()
            .filter(|line| !line.is_empty())
            .filter_map(|line| from_str(line).ok())
            .collect())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let count = input
            .iter()
            .tuples::<(_, _)>()
            .positions(|(left, right)| Day13::cmp(left, right).is_le())
            .map(|index| index + 1)
            .sum();

        Some(count)
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let markers = [json!([[2]]), json!([[6]])];

        Some(
            input
                .iter()
                .merge_by(markers.iter(), |a, b| Day13::cmp(a, b).is_le())
                .sorted_by(|a, b| Day13::cmp(a, b))
                .enumerate()
                .filter_map(|(index, value)| markers.contains(value).then_some(index + 1))
                .product(),
        )
    }
}

fn main() {
    aoc::solution!(Day13)
}
#[cfg(test)]
mod tests {
    use crate::Day13 as day_13;
    use crate::*;

    aoc::test_common!(day_13);

    aoc::test! {
     day_13:
     [case_1]
        - "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]\n"
            => Some(13)
            => Some(140);
    }
}

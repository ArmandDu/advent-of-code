use aoc::solution::SolutionError;
use aoc::Solution;

use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

const ROOT_KEY: &str = "root";
const HUMAN_KEY: &str = "humn";

struct Day21;

#[derive(Debug, Clone)]
enum MonkeyMath {
    Value(i64),
    Add(String, String),
    Sub(String, String),
    Div(String, String),
    Mul(String, String),
}

impl FromStr for MonkeyMath {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(' ').collect_tuple::<(_, _, _)>() {
            Some((a, "+", b)) => Ok(Self::Add(a.to_owned(), b.to_owned())),
            Some((a, "*", b)) => Ok(Self::Mul(a.to_owned(), b.to_owned())),
            Some((a, "-", b)) => Ok(Self::Sub(a.to_owned(), b.to_owned())),
            Some((a, "/", b)) => Ok(Self::Div(a.to_owned(), b.to_owned())),
            _ => match s.parse() {
                Ok(n) => Ok(Self::Value(n)),
                Err(_) => Err(SolutionError::ParseError),
            },
        }
    }
}

impl MonkeyMath {
    fn eval(&self, memory: &HashMap<String, MonkeyMath>) -> Option<i64> {
        use MonkeyMath::*;

        match self {
            Value(x) => Some(*x),
            Add(a, b) => Some(memory.get(a)?.eval(memory)? + memory.get(b)?.eval(memory)?),
            Sub(a, b) => Some(memory.get(a)?.eval(memory)? - memory.get(b)?.eval(memory)?),
            Div(a, b) => Some(memory.get(a)?.eval(memory)? / memory.get(b)?.eval(memory)?),
            Mul(a, b) => Some(memory.get(a)?.eval(memory)? * memory.get(b)?.eval(memory)?),
        }
    }

    fn invert(&self, x_mark: &str, y_mark: &str) -> Self {
        use MonkeyMath::*;

        match self {
            //y= a / x
            //x = a / y
            Div(lhs, rhs) if rhs == x_mark => Div(lhs.to_owned(), y_mark.to_owned()),
            //y= x / b
            //x = y / b
            Div(lhs, rhs) => {
                let rhs = if x_mark == lhs { rhs } else { lhs };

                Mul(y_mark.to_owned(), rhs.to_owned())
            }
            //y= a - x
            //x = a - y
            Sub(lhs, rhs) if rhs == x_mark => Sub(lhs.to_owned(), y_mark.to_owned()),
            //y= x - b
            //x = b - y
            Sub(lhs, rhs) => {
                let rhs = if x_mark == lhs { rhs } else { lhs };
                Add(y_mark.to_owned(), rhs.to_owned())
            }
            Add(lhs, rhs) => {
                //y= x + b
                //x = y - b
                //y= a + x
                //x = y - a
                let rhs = if x_mark == lhs { rhs } else { lhs };
                Sub(y_mark.to_owned(), rhs.to_owned())
            }
            Mul(lhs, rhs) => {
                //y= x * b
                //x = y / b
                //y= a * x
                //x = y / a
                let rhs = if x_mark == lhs { rhs } else { lhs };

                Div(y_mark.to_owned(), rhs.to_owned())
            }
            Value(_) => unreachable!("current: {}, parent: {}, node: {:?}", x_mark, y_mark, self),
        }
    }

    fn rev_path(from: &str, to: &str, memory: &HashMap<String, MonkeyMath>) -> Option<Vec<String>> {
        use MonkeyMath::*;

        let mut name = from.to_owned();
        let mut result = vec![from.to_owned()];
        loop {
            let (found, _) = memory.iter().find(|(_, op)| matches!(op, Add(a, b) | Sub(a, b) | Div(a, b) | Mul(a, b) if a == &name || b == &name))?;

            name = found.to_owned();
            result.push(found.to_owned());

            if found == to {
                break;
            }
        }

        Some(result)
    }
}

impl Solution for Day21 {
    const TITLE: &'static str = "Monkey Math";
    const DAY: u8 = 21;
    type Input = HashMap<String, MonkeyMath>;
    type P1 = i64;
    type P2 = i64;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        input
            .lines()
            .map(|line| match line.split_once(": ") {
                Some((name, expr)) => match MonkeyMath::from_str(expr) {
                    Ok(m) => Ok((name.to_owned(), m)),
                    Err(e) => Err(e),
                },
                _ => Err(SolutionError::ParseError),
            })
            .collect()
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        input.get("root")?.eval(input)
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        use MonkeyMath::*;
        let path = MonkeyMath::rev_path(HUMAN_KEY, ROOT_KEY, input)?;

        let tree = path.iter().tuple_windows::<(_, _)>().fold(
            input.clone(),
            |mut tree, (current, parent)| {
                tree.entry(current.to_owned()).and_modify(|monkey| {
                    if parent == ROOT_KEY {
                        match input.get(parent).unwrap() {
                            Add(a, b) | Sub(a, b) | Mul(a, b) | Div(a, b) => {
                                let last = path.get(path.len() - 2).unwrap();
                                let target = if last == a { b } else { a };
                                let target = input.get(target).unwrap().eval(input).unwrap();

                                *monkey = Value(target);
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        let parent_monkey = input.get(parent).unwrap();

                        *monkey = parent_monkey.invert(current, parent);
                    }
                });
                tree
            },
        );

        tree.get(HUMAN_KEY)?.eval(&tree)
    }
}

fn main() {
    aoc::solution!(Day21)
}
#[cfg(test)]
mod tests {
    use crate::Day21 as day_21;
    use crate::*;

    aoc::test_common!(day_21);

    aoc::test! {
        day_21:
        - "root: pppw + sjmn\ndbpl: 5\ncczh: sllz + lgvd\nzczc: 2\nptdq: humn - dvpt\ndvpt: 3\nlfqf: 4\nhumn: 5\nljgn: 2\nsjmn: drzm * dbpl\nsllz: 4\npppw: cczh / lfqf\nlgvd: ljgn * ptdq\ndrzm: hmdt - zczc\nhmdt: 32\n"
            => Some(152)
            => Some(301)
    }
}

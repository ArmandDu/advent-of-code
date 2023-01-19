use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error::Error;
use std::str::FromStr;

use aoc::solution::SolutionError;
use aoc::Solution;

use aoc_utils::is_flag;
use itertools::Itertools;

struct Day16;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Valve {
    name: String,
    rate: isize,
    nodes: Vec<String>,
}

impl PartialOrd<Self> for Valve {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Valve {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rate.cmp(&other.rate)
    }
}

impl FromStr for Valve {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SolutionError::ParseError;

        std::iter::once(s)
            .filter_map(|s| {
                let (part_a, part_b) = s.split_once("; ")?;
                let (id, rate) = part_a.split_once(" has flow rate=")?;
                let id = id.strip_prefix("Valve ")?;

                let nodes = part_b
                    .strip_prefix("tunnels lead to valves ")
                    .or_else(|| part_b.strip_prefix("tunnel leads to valve "))?
                    .split(", ")
                    .map(|l| l.to_owned())
                    .collect();

                Some(Self {
                    name: id.to_owned(),
                    rate: rate.parse().ok()?,
                    nodes,
                })
            })
            .next()
            .ok_or(ParseError)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
struct Index(usize);
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
struct Rate(isize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
struct Node(Index, Rate);

impl From<(Index, Rate)> for Node {
    fn from(tuple: (Index, Rate)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

impl Node {
    fn rate(&self) -> usize {
        self.1 .0 as usize
    }
    fn index(&self) -> usize {
        self.0 .0
    }
}

#[derive(Debug)]
struct Cave {
    distance_matrix: Vec<Vec<u16>>,
    graph: Vec<Node>,
    valves: Vec<Valve>,
    start: Node,
}

impl FromIterator<Valve> for Cave {
    fn from_iter<T: IntoIterator<Item = Valve>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl Cave {
    fn new(valves: Vec<Valve>) -> Self {
        let size = valves.len();
        let distance_matrix = {
            let mut matrix = vec![vec![u16::MAX; size]; size];

            valves.iter().enumerate().for_each(|(i, valve)| {
                valve
                    .nodes
                    .iter()
                    .filter_map(|name| valves.iter().position(|valve| &valve.name == name))
                    .for_each(|j| matrix[i][j] = 1);
                matrix[i][i] = 0;
            });

            (0..size)
                .cartesian_product(0..size)
                .cartesian_product(0..size)
                .for_each(|((k, i), j)| {
                    if let Some(dist) = matrix[k][i].checked_add(matrix[k][j]) {
                        matrix[i][j] = dist.min(matrix[i][j]);
                    }
                });

            matrix
        };

        let start = valves
            .iter()
            .position(|valve| valve.name == "AA")
            .map(Index)
            .unwrap();

        let graph = valves
            .iter()
            .enumerate()
            .map(|(index, valve)| (Index(index), Rate(valve.rate)))
            .filter(|(_, rate)| rate.0 > 0)
            .map(Node::from)
            .collect();

        Self {
            distance_matrix,
            graph,
            valves,
            start: Node(start, Rate(0)),
        }
    }
}

impl Cave {
    fn get_cost(&self, current: &Node, next: &Node) -> isize {
        self.distance_matrix[current.index()][next.index()] as isize
    }

    fn get_valve(&self, node: &Node) -> Option<&Valve> {
        self.valves.get(node.index())
    }

    fn max_possible_earnings(&self, remaining: &[Node], remaining_time: usize) -> usize {
        remaining
            .iter()
            .map(|node| node.rate())
            .sorted_by(|a, b| b.cmp(a))
            .zip((3..=remaining_time).rev().step_by(2))
            .fold(0, |earnings, (rate, time)| earnings + rate * (time - 2))
    }

    fn solve<F>(&self, initial_budget: usize, filter_hist: F) -> Vec<(Vec<Node>, usize)>
    where
        F: Fn(&[Node], &usize) -> bool,
    {
        let mut best = 0;
        let mut queue = BinaryHeap::new();
        let mut paths = vec![];
        let start = self.start;
        let remaining = self.graph.iter().cloned().collect_vec();

        queue.push((0, initial_budget, start, remaining, vec![]));

        while let Some((earnings, budget, current, graph, hist)) = queue.pop() {
            for next in &graph {
                let cost = self.get_cost(&current, next) + 1;
                let rate = next.rate();
                let remaining_budget = budget.saturating_add_signed(-cost);
                let earnings = earnings + remaining_budget * rate;

                let remaining_graph = graph
                    .iter()
                    .filter(|n| n.index() != next.index())
                    .cloned()
                    .collect_vec();

                let next_hist = hist
                    .iter()
                    .cloned()
                    .chain(std::iter::once(*next))
                    .collect_vec();

                if filter_hist(&remaining_graph, &remaining_budget) {
                    paths.push((next_hist.to_owned(), earnings));
                    best = best.max(earnings);
                }

                if earnings + self.max_possible_earnings(&remaining_graph, remaining_budget) > best
                {
                    queue.push((
                        earnings,
                        remaining_budget,
                        *next,
                        remaining_graph,
                        next_hist,
                    ));
                }
            }
        }

        paths
    }
}

impl Cave {
    fn print_matrix(&self) {
        println!(
            "|{} |",
            self.distance_matrix
                .iter()
                .map(|row| row
                    .iter()
                    .map(|d| if *d == u16::MAX {
                        "  ".to_owned()
                    } else {
                        format!("{:>2}", d)
                    })
                    .join(" | "))
                .join(" |\n|")
        )
    }

    fn print_paths(&self, paths: &[(Vec<Node>, usize)]) {
        println!(
            "{}",
            paths
                .iter()
                .map(|(path, earnings)| format!(
                    "[{}]: {earnings} psi",
                    path.iter()
                        .filter_map(|n| self.get_valve(n))
                        .map(|v| v.name.to_owned())
                        .join(", ")
                ))
                .join("\n")
        );
    }

    fn print_valves(&self, valves: &[Node]) {
        println!(
            "Valves: {}",
            valves
                .iter()
                .filter_map(|n| self.get_valve(n))
                .map(|v| format!("({},{})", v.name, v.rate))
                .join(", ")
        );
    }
}

impl Solution for Day16 {
    const TITLE: &'static str = "Proboscidea Volcanium";
    const DAY: u8 = 16;
    type Input = Cave;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        input.lines().map(Valve::from_str).collect()
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let take_only_leafs = |graph: &[Node], budget: &usize| graph.is_empty() || budget <= &2;

        is_print().then(|| input.print_matrix());
        is_flag("--valves").then(|| {
            input.print_valves(&input.graph);
        });

        let paths = input.solve(30, take_only_leafs);

        is_flag("-p1").then(|| input.print_paths(&paths));

        paths.iter().map(|(_, e)| *e).max()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let take_incomplete_only = &|remaining: &[Node], _: &usize| !remaining.is_empty();

        let paths = input.solve(26, take_incomplete_only);

        is_flag("-p2").then(|| input.print_paths(&paths));

        let candidates = paths
            .iter()
            .sorted_by(|(_, a), (_, b)| b.cmp(a))
            .collect::<Vec<_>>();

        candidates
            .iter()
            .cartesian_product(candidates.iter())
            .filter(|((a, _), (b, _))| a.iter().all(|lhs| !b.contains(lhs)))
            //not sure why taking the first 10000 works.
            // but I need a way to avoid iterating through all permutations
            .take(10_000)
            .map(|((_, a), (_, b))| *a + *b)
            .max()
    }
}

fn is_print() -> bool {
    is_flag("--print")
}

const EXAMPLE_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA\nValve CC has flow rate=2; tunnels lead to valves DD, BB\nValve DD has flow rate=20; tunnels lead to valves CC, AA, EE\nValve EE has flow rate=3; tunnels lead to valves FF, DD\nValve FF has flow rate=0; tunnels lead to valves EE, GG\nValve GG has flow rate=0; tunnels lead to valves FF, HH\nValve HH has flow rate=22; tunnel leads to valve GG\nValve II has flow rate=0; tunnels lead to valves AA, JJ\nValve JJ has flow rate=21; tunnel leads to valve II\n";

fn main() -> Result<(), Box<dyn Error>> {
    match is_flag("--example") {
        true => {
            Day16::test_part1(EXAMPLE_INPUT)?;
            Day16::test_part2(EXAMPLE_INPUT)?;
        }
        _ => {
            aoc::solution!(Day16);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Day16 as day_16;
    use crate::*;

    aoc::test_common!(day_16);

    aoc::test! {
        day_16:
        - EXAMPLE_INPUT
            => Some(1651)
            => Some(1707)
    }
}

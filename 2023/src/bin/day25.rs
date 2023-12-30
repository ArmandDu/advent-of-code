use std::collections::{HashMap, HashSet};

use aoc::solution::SolutionError;
use aoc::Solution;

pub struct Day25;

impl Solution for Day25 {
    const TITLE: &'static str = "Snowverload";
    const DAY: u8 = 25;
    type Input = HashMap<String, HashSet<String>>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        input
            .lines()
            .try_fold(HashMap::<_, HashSet<_>>::new(), |mut graph, line| {
                let (node, children) = line.split_once(": ")?;

                for child in children.split_whitespace() {
                    let node = node.trim().to_owned();
                    let child = child.trim().to_owned();

                    graph.entry(node.clone()).or_default().insert(child.clone());
                    graph.entry(child).or_default().insert(node);
                }

                Some(graph)
            })
            .ok_or(SolutionError::ParseError)
    }

    fn part1(original: &Self::Input) -> Option<Self::P1> {
        let nodes: HashSet<_> = original.keys().cloned().collect();
        let mut derived = nodes.clone();

        while (derived
            .iter()
            .map(|k| original.get(k).unwrap().difference(&derived).count())
            .sum::<usize>())
            != 3
        {
            let best_candidate = derived
                .iter()
                .max_by_key(|&k| original.get(k).unwrap().difference(&derived).count())
                .cloned()?;

            derived.remove(&best_candidate);
        }

        Some(derived.len() * nodes.difference(&derived).count())
    }

    fn part2(_input: &Self::Input) -> Option<Self::P2> {
        None
    }
}

aoc::run!(Day25);

aoc::example! {
    [Day25]
    sample: "jqt: rhn xhk nvd\r\nrsh: frs pzl lsr\r\nxhk: hfx\r\ncmg: qnr nvd lhk bvb\r\nrhn: xhk bvb hfx\r\nbvb: xhk hfx\r\npzl: lsr hfx nvd\r\nqnr: nvd\r\nntq: jqt hfx bvb xhk\r\nnvd: lhk\r\nlsr: lhk\r\nrzs: qnr cmg lsr rsh\r\nfrs: qnr lhk lsr\r\n"
        => Some(54)
        => None
}

use aoc::solution::SolutionError;
use aoc::Solution;
use aoc_utils::lcm;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct GhostMap {
    directions: Vec<char>,
    map: HashMap<String, (String, String)>,
}

impl GhostMap {
    fn next_position(&self, position: &str, direction: &char) -> Option<String> {
        let (left, right) = match self.map.get(position) {
            Some(coord) => coord,
            _ => return None,
        };

        Some(
            match direction {
                'L' => left,
                _ => right,
            }
            .to_owned(),
        )
    }

    fn find_exit(&self, mut position: String, is_exit: impl Fn(&String) -> bool) -> Option<usize> {
        self.directions
            .iter()
            .cycle()
            .find_position(|&dir| {
                match self.next_position(&position, dir) {
                    Some(next_position) => position = next_position,
                    _ => return false,
                }

                is_exit(&position)
            })
            .map(|(index, _)| index + 1)
    }
}

impl FromStr for GhostMap {
    type Err = SolutionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace('\r', "");

        let (directions, map) = input.split_once("\n\n").ok_or(SolutionError::ParseError)?;

        let map = map
            .lines()
            .map(|line| {
                let (name, nodes) = line.split_once(" = ").ok_or(SolutionError::ParseError)?;
                let nodes = nodes.replace(['(', ')'], "");

                let (left, right) = nodes
                    .trim()
                    .split_once(", ")
                    .ok_or(SolutionError::ParseError)?;

                Ok((name.trim().to_owned(), (left.to_owned(), right.to_owned())))
            })
            .collect::<Result<_, SolutionError>>()?;

        Ok(GhostMap {
            directions: directions.chars().collect(),
            map,
        })
    }
}

struct Day08;

impl Solution for Day08 {
    const TITLE: &'static str = "Haunted Wasteland";
    const DAY: u8 = 8;
    type Input = GhostMap;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        GhostMap::from_str(input)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let position = "AAA".to_owned();

        if !input.map.contains_key(&position) {
            return None;
        }

        input.find_exit(position, |exit| exit == "ZZZ")
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let positions = input
            .map
            .keys()
            .filter(|key| key.ends_with('A'))
            .cloned()
            .map(|position| input.find_exit(position, |exit| exit.ends_with('Z')))
            .collect::<Option<Vec<_>>>()?;

        positions.iter().cloned().reduce(lcm)
    }
}

aoc::run!(Day08);

aoc::example! {
    [Day08]
    two_steps: "RL\r\n\r\nAAA = (BBB, CCC)\r\nBBB = (DDD, EEE)\r\nCCC = (ZZZ, GGG)\r\nDDD = (DDD, DDD)\r\nEEE = (EEE, EEE)\r\nGGG = (GGG, GGG)\r\nZZZ = (ZZZ, ZZZ)\r\n"
        => Some(2)
        => Some(2)
    six_steps: "LLR\r\n\r\nAAA = (BBB, BBB)\r\nBBB = (AAA, ZZZ)\r\nZZZ = (ZZZ, ZZZ)\r\n"
        => Some(6)
        => Some(6)
    ghost_steps: "LR\r\n\r\n11A = (11B, XXX)\r\n11B = (XXX, 11Z)\r\n11Z = (11B, XXX)\r\n22A = (22B, XXX)\r\n22B = (22C, 22C)\r\n22C = (22Z, 22Z)\r\n22Z = (22B, 22B)\r\nXXX = (XXX, XXX)\r\n"
        => None
        => Some(6)
}

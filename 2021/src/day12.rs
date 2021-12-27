use std::collections::HashMap;
use year2021::Solution;

pub struct Day12;

impl Solution<usize, usize> for Day12 {
    const DAY: u32 = 12;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Passage Pathing";
    type Input = HashMap<String, Vec<String>>;

    fn part1(input: &Self::Input) -> Option<usize> {
        let solves = Self::backtrack(input, "start", "start", "end", &mut Vec::new(), false);

        Some(solves.len())
    }

    fn part2(input: &Self::Input) -> Option<usize> {
        let solves = Self::backtrack(input, "start", "start", "end", &mut Vec::new(), true);

        Some(solves.len())
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        Ok(input
            .trim()
            .lines()
            .filter_map(|line| line.split_once("-"))
            .fold(HashMap::new(), |mut acc, (enter, exit)| {
                let enter = enter.to_string();
                let exit = exit.to_string();

                acc.entry(enter.clone())
                    .or_insert_with(Vec::new)
                    .push(exit.clone());

                acc.entry(exit).or_insert_with(Vec::new).push(enter);
                acc
            }))
    }
}

impl Day12 {
    fn backtrack(
        nodes: &HashMap<String, Vec<String>>,
        start: &str,
        current: &str,
        end: &str,
        buffer: &mut Vec<String>,
        mut allow_second_pass: bool,
    ) -> Vec<Vec<String>> {
        let mut solves = vec![];
        let original_allow_second_path = allow_second_pass;

        if current == end {
            let mut res = buffer.clone();

            res.push(current.to_string());
            return vec![res];
        }

        if let Some(paths) = nodes.get(current) {
            for path in paths {
                if buffer.contains(path) && path.to_uppercase() != *path {
                    if allow_second_pass && path != start {
                        allow_second_pass = false;
                    } else {
                        continue;
                    }
                }

                buffer.push(current.to_string());

                solves.append(&mut Self::backtrack(
                    nodes,
                    start,
                    path,
                    end,
                    buffer,
                    allow_second_pass,
                ));

                buffer.pop();
                allow_second_pass = original_allow_second_path;
            }
        }

        solves
    }
}

use aoc::Solution;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day14;

impl Solution<usize, usize> for Day14 {
    const DAY: u32 = 14;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Extended Polymerization";
    type Input = (Vec<(char, char)>, HashMap<(char, char), char>);

    fn part1(input: &Self::Input) -> Option<usize> {
        Self::grow_polymer(&input, 10)
    }

    fn part2(input: &Self::Input) -> Option<usize> {
        Self::grow_polymer(&input, 40)
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        let (template, rules) = input.trim().split_once("\n\n").expect("Invalid input");

        Ok((
            template.chars().tuple_windows().collect(),
            rules
                .lines()
                .map(|line| {
                    let (pattern, insert) = line.split_once(" -> ").expect("Invalid input");

                    (
                        pattern.chars().collect_tuple().unwrap(),
                        insert.as_bytes()[0] as char,
                    )
                })
                .collect(),
        ))
    }
}

impl Day14 {
    fn grow_polymer(
        input: &(Vec<(char, char)>, HashMap<(char, char), char>),
        iterations: usize,
    ) -> Option<usize> {
        let (template, rules) = input;

        // count all initial pairs occurrences.
        // nb: a map(|key| (key, 1)).collect::<HashMap<_,_>>() might not be enough if a pair appear more than once initially
        let initial_count: HashMap<_, _> =
            template.iter().fold(HashMap::new(), |mut acc, &pair| {
                *acc.entry(pair).or_insert(0) += 1;

                acc
            });

        //then we iterate a couple of lifetimes...
        let final_count = (0..iterations).fold(initial_count, |count, _| {
            count
                .iter()
                .fold(HashMap::new(), |mut new_count, (&key, count)| {
                    // then everytime, a new letter is inserted, two new pairs are created.
                    // example:
                    // Initial: NN = 1, NC = 1, CB = 1
                    // NN -> C generates two new pairs NC and CN. so the new set would have NC = 1 and CN = 1
                    // Then NC -> B generates two new pairs NB and BC. So  NC = 1, CN = 1, NB = 1, BC = 1
                    // Finally CB -> H generates CH and HB. NC = 1, CN = 1, NB = 1, BC = 1, CH = 1, HB = 1
                    // by having a new fresh set everytime, we don't need to keep track of the destroyed sets
                    // (by inserting a C between NN, we destroy NN and create NC and CN)
                    // starting from the new set, we make sure to keep the current count
                    // because if we had 2 NC for example, we know it would create 2 NB and 2 BC and the 2 NC would be destroyed
                    // then we repeat this process for each iteration.
                    if let Some(&c) = rules.get(&key) {
                        *new_count.entry((key.0, c)).or_insert(0) += count;
                        *new_count.entry((c, key.1)).or_insert(0) += count;
                    }

                    new_count
                })
        });

        // then, we have our pairs but we need to count how many time each letter appear.
        // because we are dealing with windows, a list of NN, NC, CB, we should count only 2 N, 1 C and 1 B (in the window, N appears 3 times, C 2 times)
        // for that, we can count only the first letter of each pair, which gives N, N, C.
        // We would be missing the last letter so we have to add it manually later...
        let mut letter_count =
            final_count
                .iter()
                .fold(HashMap::new(), |mut acc, (&key, amount)| {
                    *acc.entry(key.0).or_insert(0) += amount;

                    acc
                });

        // We will always be missing the last letter in the sequence so we have have to count it manually.
        if let Some(&(_, last)) = template.last() {
            *letter_count.entry(last).or_insert(0) += 1;
        }

        //Lastly, we get the minmax and return the difference
        letter_count
            .values()
            .minmax()
            .into_option()
            .and_then(|(min, max)| Some(max - min))
    }
}

fn main() {
    Day14::run(include_str!("../../data/day14_input"));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C";
    #[test]
    fn day14() {
        Day14::test(INPUT, Some(1588), Some(2188189693529));
    }
}

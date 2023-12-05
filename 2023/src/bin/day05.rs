use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;
use std::str::FromStr;

struct Day05;

type MapFormat = (i64, i64, i64);
#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Vec<MapFormat>>,
}

impl Almanac {
    fn get_seed_position(&self, seed: i64) -> i64 {
        self.maps.iter().fold(seed, |position, ranges| {
            ranges
                .iter()
                .find(|(_, source, range)| position >= *source && position < source + range)
                .map(|(dest, source, _)| (dest - source) + position)
                .unwrap_or(position)
        })
    }
}

impl FromStr for Almanac {
    type Err = SolutionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace('\r', "");
        let mut it = input.split("\n\n");

        let seeds = it
            .next()
            .ok_or(SolutionError::ParseError)?
            .split_whitespace()
            .skip(1)
            .map(|seed| seed.parse())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| SolutionError::ParseError)?;

        let maps = it
            .map(|map| {
                map.split('\n')
                    .skip(1)
                    .filter_map(|row| {
                        row.split_whitespace()
                            .filter_map(|num| num.parse().ok())
                            .collect_tuple::<MapFormat>()
                    })
                    .collect_vec()
            })
            .collect_vec();

        Ok(Almanac { seeds, maps })
    }
}

impl Solution for Day05 {
    const TITLE: &'static str = "If You Give A Seed A Fertilizer";
    const DAY: u8 = 5;
    type Input = Almanac;
    type P1 = i64;
    type P2 = i64;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        input.parse()
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        input
            .seeds
            .iter()
            .map(|&seed| input.get_seed_position(seed))
            .min()
    }

    fn part2(_input: &Self::Input) -> Option<Self::P2> {
        None
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
        [example]
        - "seeds: 79 14 55 13\r\n\r\nseed-to-soil map:\r\n50 98 2\r\n52 50 48\r\n\r\nsoil-to-fertilizer map:\r\n0 15 37\r\n37 52 2\r\n39 0 15\r\n\r\nfertilizer-to-water map:\r\n49 53 8\r\n0 11 42\r\n42 0 7\r\n57 7 4\r\n\r\nwater-to-light map:\r\n88 18 7\r\n18 25 70\r\n\r\nlight-to-temperature map:\r\n45 77 23\r\n81 45 19\r\n68 64 13\r\n\r\ntemperature-to-humidity map:\r\n0 69 1\r\n1 0 69\r\n\r\nhumidity-to-location map:\r\n60 56 37\r\n56 93 4\r\n"
            => Some(35)
            => Some(46);

    }
}

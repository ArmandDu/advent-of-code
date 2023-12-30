use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;
use std::str::FromStr;

enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl FromStr for Cube {
    type Err = SolutionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (count, color) = value
            .trim()
            .split_once(' ')
            .ok_or(SolutionError::ParseError)?;
        let count = count.parse().map_err(|_| SolutionError::ParseError)?;

        match color {
            "red" => Ok(Cube::Red(count)),
            "green" => Ok(Cube::Green(count)),
            "blue" => Ok(Cube::Blue(count)),
            _ => Err(SolutionError::ParseError),
        }
    }
}

struct Game {
    id: u32,
    cubes: Vec<Cube>,
}

impl FromStr for Game {
    type Err = SolutionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (game_id, game) = value.split_once(':').ok_or(SolutionError::ParseError)?;
        let game_id = game_id
            .replace("Game ", "")
            .parse()
            .map_err(|_| SolutionError::ParseError)?;

        let cubes = game
            .split(&[',', ';'])
            .map(Cube::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Game { id: game_id, cubes })
    }
}

struct Day02;

impl Solution for Day02 {
    const TITLE: &'static str = "Cube Conundrum";
    const DAY: u8 = 2;
    type Input = Vec<Game>;
    type P1 = u32;
    type P2 = u32;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        input.lines().map(|line| line.parse()).collect()
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        input
            .iter()
            .filter(|game| {
                game.cubes.iter().all(|cube| match cube {
                    Cube::Red(qty) => qty <= &12,
                    Cube::Green(qty) => qty <= &13,
                    Cube::Blue(qty) => qty <= &14,
                })
            })
            .map(|game| game.id)
            .sum1()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        input
            .iter()
            .map(|game| {
                let (r, g, b) = game
                    .cubes
                    .iter()
                    .fold((0, 0, 0), |(r, g, b), cube| match cube {
                        Cube::Red(qty) => (r.max(*qty), g, b),
                        Cube::Green(qty) => (r, g.max(*qty), b),
                        Cube::Blue(qty) => (r, g, b.max(*qty)),
                    });

                r * g * b
            })
            .sum1()
    }
}

fn main() {
    aoc::solution!(Day02)
}
#[cfg(test)]
mod tests {
    use crate::Day02 as day_02;
    use crate::*;

    aoc::test_common!(day_02);

    aoc::test! {
        day_02:
        [digits]
        - "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\r\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\r\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\r\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\r\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\r\n"
            => Some(8)
            => Some(2286);

    }
}

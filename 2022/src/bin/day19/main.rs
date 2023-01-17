use aoc::solution::SolutionError;
use aoc::Solution;

use rayon::prelude::*;
use regex::{Captures, Regex};

use std::error::Error;

use aoc_utils::is_flag;
use blueprint::Blueprint;

mod blueprint;
mod factory;
mod material;

struct Day19;

impl Solution for Day19 {
    const TITLE: &'static str = "Not Enough Minerals";
    const DAY: u8 = 19;
    type Input = Vec<Blueprint>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        use material::Material::*;
        use material::{MatCount, Material, RobotCost};
        let re = Regex::new(
            r"Blueprint \d+:\s+Each ore robot costs (?P<ore_ore>\d+) ore\.\s+Each clay robot costs (?P<clay_ore>\d+) ore\.\s+Each obsidian robot costs (?P<obs_ore>\d+) ore and (?P<obs_clay>\d+) clay\.\s+Each geode robot costs (?P<geo_ore>\d+) ore and (?P<geo_obs>\d+) obsidian\.",
        );
        let re = re.map_err(|_| SolutionError::ParseError)?;

        fn parse(
            robot: Material,
            material: Material,
            c: &Captures,
        ) -> Option<(Material, MatCount)> {
            use material::Material::*;

            let key = match (robot, material) {
                (Ore, Ore) => "ore_ore",
                (Clay, Ore) => "clay_ore",
                (Obsidian, Ore) => "obs_ore",
                (Obsidian, Clay) => "obs_clay",
                (Geode, Ore) => "geo_ore",
                (Geode, Obsidian) => "geo_obs",
                _ => unreachable!(),
            };

            Some((material, c[key].parse::<usize>().ok()?.into()))
        }

        Ok(input
            .lines()
            .enumerate()
            .filter_map(|(index, line)| {
                let found = re.captures(line)?;

                let ore_robot = RobotCost::new(Ore, [parse(Ore, Ore, &found)?]);
                let clay_robot = RobotCost::new(Clay, [parse(Clay, Ore, &found)?]);
                let obsidian_robot = RobotCost::new(
                    Obsidian,
                    [
                        parse(Obsidian, Ore, &found)?,
                        parse(Obsidian, Clay, &found)?,
                    ],
                );
                let geode_robot = RobotCost::new(
                    Geode,
                    [parse(Geode, Ore, &found)?, parse(Geode, Obsidian, &found)?],
                );

                Some(Blueprint::new(
                    index + 1,
                    vec![ore_robot, clay_robot, obsidian_robot, geode_robot],
                ))
            })
            .collect())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let budget = 24;

        Some(
            input
                .par_iter()
                .enumerate()
                .fold(
                    || 0,
                    |sum, (index, bp)| sum + (index + 1) * bp.solve(budget),
                )
                .sum(),
        )
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let budget = 32;

        Some(
            input
                .iter()
                .take(3)
                .par_bridge()
                .map(|bp| bp.solve(budget))
                .product(),
        )
    }
}

const EXAMPLE_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\nBlueprint 2:  Each ore robot costs 2 ore.  Each clay robot costs 3 ore.  Each obsidian robot costs 3 ore and 8 clay.  Each geode robot costs 3 ore and 12 obsidian.";

fn main() -> Result<(), Box<dyn Error>> {
    match is_flag("--example") {
        true => {
            Day19::test_part1(EXAMPLE_INPUT)?;
            Day19::test_part2(EXAMPLE_INPUT)?;
        }
        false => {
            aoc::solution!(Day19);
        }
    };
    Ok(())
}
#[cfg(test)]
mod tests {
    use crate::Day19 as day19;
    use crate::*;

    aoc::test_common!(day19);

    aoc::test! {
        day19:
        - EXAMPLE_INPUT
            => Some(33)
            => Some(62*56)
    }
}

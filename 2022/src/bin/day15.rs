use std::collections::HashMap;

use aoc::Solution;
use itertools::Itertools;
use rayon::prelude::*;

use shared::Point as PointT;

type Point = PointT<i32>;

struct Day15<const TARGET: usize, const AREA: usize>;

#[derive(Debug, Copy, Clone)]
enum Tile {
    Sensor(i32),
    Beacon,
}

impl<const T: usize, const A: usize> Day15<T, A> {
    fn parse_point(source: &str) -> Option<Point> {
        let (x, y) = source.split_once(", ")?;

        Some(Point::new(x[2..].parse().ok()?, y[2..].parse().ok()?))
    }

    fn manhattan_distance(lhs: &Point, rhs: &Point) -> i32 {
        (lhs.x().abs_diff(rhs.x()) + lhs.y().abs_diff(rhs.y())) as i32
    }
}

impl<const TARGET: usize, const AREA: usize> Solution for Day15<TARGET, AREA> {
    const TITLE: &'static str = "Beacon Exclusion Zone";
    const DAY: u8 = 15;
    type Input = HashMap<Point, Tile>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Ok(input
            .lines()
            .filter_map(|line| {
                let (sensor, beacon) = line.split_once(": ")?;

                let sensor = sensor.strip_prefix("Sensor at ")?;
                let beacon = beacon.strip_prefix("closest beacon is at ")?;

                let sensor = Day15::<TARGET, AREA>::parse_point(sensor)?;
                let beacon = Day15::<TARGET, AREA>::parse_point(beacon)?;

                Some([
                    (
                        sensor,
                        Tile::Sensor(Day15::<TARGET, AREA>::manhattan_distance(&sensor, &beacon)),
                    ),
                    (beacon, Tile::Beacon),
                ])
            })
            .flatten()
            .collect())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let (min, max) = input
            .iter()
            .filter_map(|(coord, tile)| match tile {
                Tile::Sensor(radius) => Some([coord.x() - radius, coord.x() + radius]),
                _ => None,
            })
            .flatten()
            .minmax()
            .into_option()?;

        Some(
            (min..max)
                .map(|x| Point::new(x, TARGET as i32))
                .filter(|target| !input.contains_key(target))
                .filter(|target| {
                    input.iter().any(|(current, tile)| match tile {
                        Tile::Sensor(radius) => {
                            Day15::<TARGET, AREA>::manhattan_distance(target, current) <= *radius
                        }
                        _ => false,
                    })
                })
                .count(),
        )
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let sensors: Vec<_> = input
            .iter()
            .filter(|(_, tile)| matches!(tile, Tile::Sensor(_)))
            .collect();

        let point = sensors.iter().find_map(|(sensor, tile)| match tile {
            Tile::Sensor(radius) => sensor
                .circle(radius + 1)
                .into_par_iter()
                .filter(|circle| {
                    let (x, y) = circle.xy();
                    let min = 0;
                    let max = AREA as i32;

                    min <= x && x <= max && min <= y && y <= max
                })
                .filter(|circle| !input.contains_key(circle))
                .find_any(|circle| {
                    sensors
                        .iter()
                        .all(|(other, other_sensor)| match other_sensor {
                            Tile::Sensor(test_radius) => {
                                Day15::<TARGET, AREA>::manhattan_distance(other, circle)
                                    > *test_radius
                            }
                            _ => unreachable!(),
                        })
                }),
            _ => unreachable!(),
        })?;

        Some(point.x() as usize * 4000000 + point.y() as usize)
    }
}

fn main() {
    type Day = Day15<2000000, 4000000>;
    // type Day = Day15<10, 20>;

    aoc::solution!(Day)
}
#[cfg(test)]
mod tests {
    use crate::Day15;
    use crate::*;

    #[allow(non_camel_case_types)]
    type day_15 = Day15<10, 20>;

    aoc::test_common!(day_15);

    aoc::test! {
        day_15:
        - "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16\nSensor at x=13, y=2: closest beacon is at x=15, y=3\nSensor at x=12, y=14: closest beacon is at x=10, y=16\nSensor at x=10, y=20: closest beacon is at x=10, y=16\nSensor at x=14, y=17: closest beacon is at x=10, y=16\nSensor at x=8, y=7: closest beacon is at x=2, y=10\nSensor at x=2, y=0: closest beacon is at x=2, y=10\nSensor at x=0, y=11: closest beacon is at x=2, y=10\nSensor at x=20, y=14: closest beacon is at x=25, y=17\nSensor at x=17, y=20: closest beacon is at x=21, y=22\nSensor at x=16, y=7: closest beacon is at x=15, y=3\nSensor at x=14, y=3: closest beacon is at x=15, y=3\nSensor at x=20, y=1: closest beacon is at x=15, y=3\n"
            => Some(26)
            => Some(56000011)
    }
}

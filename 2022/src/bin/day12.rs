use aoc::solution::SolutionError;
use aoc::Solution;
use aoc_utils::dijkstra::{self, Boundaries, Coord};

struct Day12;

#[derive(Debug)]
struct Hill {
    start: Coord,
    top: Coord,
    boundaries: Boundaries,
    map: Vec<Vec<char>>,
}

impl Hill {
    fn get_cost(&self, current: &Coord, next: &Coord) -> Option<i32> {
        let current = self.get(current)?;
        let next = self.get(next)?;

        match *next as i32 - *current as i32 {
            x if x < 2 => Some(1),
            _ => None,
        }
    }

    fn get(&self, coord: &Coord) -> Option<&char> {
        self.map.get(coord.y)?.get(coord.x)
    }
}

impl Solution for Day12 {
    const TITLE: &'static str = "Hill Climbing Algorithm";
    const DAY: u8 = 12;
    type Input = Hill;
    type P1 = i32;
    type P2 = i32;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        let mut start = Coord { x: 0, y: 0 };
        let mut end = Coord { x: 0, y: 0 };

        let map: Vec<Vec<_>> = input
            .lines()
            .enumerate()
            .map(|(j, line)| {
                line.chars()
                    .enumerate()
                    .map(|(i, c)| match c {
                        'S' => {
                            start.x = i;
                            start.y = j;
                            'a'
                        }
                        'E' => {
                            end.x = i;
                            end.y = j;
                            'z'
                        }
                        _ => c,
                    })
                    .collect()
            })
            .collect();

        Ok(Hill {
            start,
            top: end,
            boundaries: Boundaries {
                x: 0,
                y: 0,
                height: map.len(),
                width: map.first().ok_or(SolutionError::ParseError)?.len(),
            },
            map,
        })
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        dijkstra::solve(
            &input.start,
            &input.boundaries,
            |current, next| input.get_cost(current, next),
            |current| current == &input.top,
        )
        .map(|(_, steps)| steps)
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        dijkstra::solve(
            &input.top,
            &input.boundaries,
            |current, next| input.get_cost(next, current),
            |current| input.get(current) == Some(&'a'),
        )
        .map(|(_, steps)| steps)
    }
}

fn main() {
    aoc::solution!(Day12)
}
#[cfg(test)]
mod tests {
    use crate::Day12 as day_12;
    use crate::*;

    aoc::test_common!(day_12);

    aoc::test! {
     day_12:
     [case_1]
        - "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n"
            => Some(31)
            => Some(29);
    }
}

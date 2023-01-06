use aoc::solution::SolutionError;
use aoc::Solution;
use aoc_utils::pathfinding::{dijkstra, Graph};

struct Day12;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Hill {
    start: Coord,
    top: Coord,
    width: usize,
    height: usize,
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
    fn is_in_bound(&self, coord: &Coord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    fn get(&self, coord: &Coord) -> Option<&char> {
        self.map.get(coord.y)?.get(coord.x)
    }
}

struct Expedition<'a, T: Fn(&Coord) -> bool> {
    hill: &'a Hill,
    get_target: T,
    start: Coord,
}

impl<'a, T: Fn(&Coord) -> bool> Expedition<'a, T> {
    pub fn new(hill: &'a Hill, start: Coord, get_target: T) -> Self {
        Self {
            hill,
            get_target,
            start,
        }
    }

    fn as_index(&self, coord: &Coord) -> usize {
        let Hill { width, .. } = self.hill;

        coord.y * width + coord.x
    }

    fn as_coord(&self, index: usize) -> Coord {
        let Hill { width, .. } = self.hill;

        Coord {
            x: index % width,
            y: index / width,
        }
    }

    fn get_cost(&self, current: usize, next: usize) -> Option<i32> {
        let current = self.as_coord(current);
        let next = self.as_coord(next);

        self.hill.get_cost(&current, &next)
    }
}

impl<T: Fn(&Coord) -> bool> Graph<usize> for Expedition<'_, T> {
    fn start(&self) -> Option<usize> {
        Some(self.as_index(&self.start))
    }

    fn adjacent(&self, node: &usize) -> Option<Vec<usize>> {
        const NEIGHBORS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let Coord { x, y } = self.as_coord(*node);

        Some(
            NEIGHBORS
                .iter()
                .filter_map(|&(dx, dy)| {
                    Some(Coord {
                        x: x.checked_add_signed(dx)?,
                        y: y.checked_add_signed(dy)?,
                    })
                })
                .filter(|coord| self.hill.is_in_bound(coord))
                .map(|coord| self.as_index(&coord))
                .collect(),
        )
    }

    fn is_target(&self, node: &usize) -> bool {
        let test = &self.get_target;

        test(&self.as_coord(*node))
    }

    fn estimated_size(&self) -> usize {
        self.hill.width * self.hill.height
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
            height: map.len(),
            width: map.first().ok_or(SolutionError::ParseError)?.len(),
            map,
        })
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let expedition = Expedition::new(input, input.start, |current| current == &input.top);

        dijkstra::solve(&expedition, |current, next| {
            expedition.get_cost(*current, *next)
        })
        .map(|(cost, _)| cost)
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let expedition =
            Expedition::new(input, input.top, |current| input.get(current) == Some(&'a'));

        dijkstra::solve(&expedition, |current, next| {
            expedition.get_cost(*next, *current)
        })
        .map(|(cost, _)| cost)
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

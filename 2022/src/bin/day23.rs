use aoc::Solution;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

struct Day23;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Elf(i64, i64);

impl From<(i64, i64)> for Elf {
    fn from((x, y): (i64, i64)) -> Self {
        Self(x, y)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn iter() -> impl Iterator<Item = Direction> {
        use Direction::*;

        [N, NE, E, SE, S, SW, W, NW].into_iter()
    }
}

impl From<Direction> for (i64, i64) {
    fn from(d: Direction) -> Self {
        use Direction::*;

        match d {
            N => (0, -1),
            NE => (1, -1),
            E => (1, 0),
            SE => (1, 1),
            S => (0, 1),
            SW => (-1, 1),
            W => (-1, 0),
            NW => (-1, -1),
        }
    }
}

impl Elf {
    fn look_around(&self, grid: &HashSet<Elf>) -> HashMap<Direction, bool> {
        Direction::iter()
            .map(|dir| {
                let (dx, dy) = dir.into();
                let next = (self.0 + dx, self.1 + dy).into();

                (dir, grid.contains(&next))
            })
            .collect()
    }

    fn make_proposal(
        &self,
        grid: &HashSet<Elf>,
        lookup: &VecDeque<[Direction; 3]>,
    ) -> Option<(i64, i64)> {
        let around = self.look_around(grid);

        if around.iter().all(|(_, &occupied)| !occupied) {
            None
        } else {
            lookup
                .iter()
                .find(|dirs| {
                    dirs.iter()
                        .all(|d| around.get(d).map(|&occupied| !occupied).unwrap_or(false))
                })
                .and_then(|dirs| dirs.first())
                .map(|&dir| {
                    let (dx, dy) = dir.into();

                    (self.0 + dx, self.1 + dy)
                })
        }
    }
}

impl Day23 {
    fn get_fov_list() -> VecDeque<[Direction; 3]> {
        VecDeque::from([
            [Direction::N, Direction::NW, Direction::NE],
            [Direction::S, Direction::SW, Direction::SE],
            [Direction::W, Direction::NW, Direction::SW],
            [Direction::E, Direction::NE, Direction::SE],
        ])
    }

    fn get_area(input: &HashSet<Elf>) -> ((i64, i64), (i64, i64)) {
        input.iter().fold(
            match input.len() {
                0 => ((0, 0), (0, 0)),
                _ => ((i64::MAX, i64::MIN), (i64::MAX, i64::MIN)),
            },
            |(u, v), &Elf(x, y)| ((u.0.min(x), u.1.max(x + 1)), (v.0.min(y), v.1.max(y + 1))),
        )
    }

    fn round(mut pool: HashSet<Elf>, directions: &VecDeque<[Direction; 3]>) -> HashSet<Elf> {
        let mut proposals: HashMap<_, Vec<_>> = HashMap::new();

        for elf in &pool {
            if let Some(proposal) = elf.make_proposal(&pool, directions) {
                proposals.entry(proposal).or_default().push(*elf);
            }
        }

        proposals
            .iter()
            .filter_map(|(proposal, queue)| match queue.len() {
                1 => queue.first().map(|elf| (proposal, elf)),
                _ => None,
            })
            .for_each(|(&proposal, elf)| {
                pool.remove(elf);
                pool.insert(proposal.into());
            });

        pool
    }

    fn print(input: &HashSet<Elf>, zoom: i64) {
        let (x_range, y_range) = Self::get_area(input);

        let grid = ((y_range.0 - zoom)..(y_range.1 + zoom))
            .map(|y| {
                ((x_range.0 - zoom)..(x_range.1 + zoom))
                    .map(|x| match input.contains(&(x, y).into()) {
                        true => "#",
                        false => ".",
                    })
                    .collect::<String>()
            })
            .join("\n");

        println!("{}", grid);
    }
}

impl Solution for Day23 {
    const TITLE: &'static str = "Unstable Diffusion";
    const DAY: u8 = 23;
    type Input = HashSet<Elf>;
    type P1 = u64;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Ok(input
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars().enumerate().filter_map(move |(x, c)| match c {
                    '#' => Some((x as i64, y as i64).into()),
                    _ => None,
                })
            })
            .collect())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let mut directions = Day23::get_fov_list();

        is_print().then(|| Day23::print(input, 1));
        let pool = (1..=10).fold(input.to_owned(), |pool, round| {
            let next_pool = Day23::round(pool, &directions);

            is_print().then(|| {
                println!("Round {}:", round);
                Day23::print(&next_pool, 1);
            });

            directions.rotate_left(1);
            next_pool
        });

        let (x_range, y_range) = Day23::get_area(&pool);

        Some(x_range.1.abs_diff(x_range.0) * y_range.1.abs_diff(y_range.0) - pool.len() as u64)
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let mut directions = Day23::get_fov_list();
        let mut pool = input.to_owned();

        (1..).find(|_| {
            let next_pool = Day23::round(pool.to_owned(), &directions);

            if next_pool == pool {
                true
            } else {
                directions.rotate_left(1);
                pool = next_pool;

                false
            }
        })
    }
}

fn is_print() -> bool {
    is_flag("--print")
}

fn is_flag(flag: &str) -> bool {
    std::env::args().any(|arg| arg.as_str() == flag)
}

fn main() {
    aoc::solution!(Day23);
}
#[cfg(test)]
mod tests {
    use crate::Day23 as day_23;
    use crate::*;

    aoc::test_common!(day_23);

    aoc::test! {
     day_23:
     [larger_exaple]
        - "....#..\n..###.#\n#...#.#\n.#...##\n#.###..\n##.#.##\n.#..#..\n" => Some(110) => Some(20);
    }
}

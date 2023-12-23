use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use aoc::solution::SolutionError;
use aoc::Solution;

use aoc_utils::is_flag;

struct Day21<const STEPS: usize>;

struct Garden(Vec<Vec<char>>);

impl FromStr for Garden {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Garden(
            s.lines().map(|line| line.chars().collect()).collect(),
        ))
    }
}

impl Garden {
    fn find(
        &self,
        predicate: impl Fn(((usize, usize), &char)) -> bool,
    ) -> Option<((usize, usize), &char)> {
        self.0.iter().enumerate().find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, tile)| predicate(((x, y), tile)).then_some(((x, y), tile)))
        })
    }

    fn get(&self, coord: &(usize, usize)) -> Option<&char> {
        self.0.get(coord.1)?.get(coord.0)
    }
}

impl Garden {
    fn solve(&self, max_steps: usize) -> Option<HashSet<(usize, usize)>> {
        let start = self.start()?;
        let mut visited = HashSet::from([start]);
        let mut accepted = HashSet::new();

        let mut queue = VecDeque::from([(start, max_steps)]);

        while let Some((current, steps)) = queue.pop_front() {
            if steps % 2 == max_steps % 2 {
                accepted.insert(current);
            }

            if steps > 0 {
                for next in self.adjacent(&current)? {
                    if visited.insert(next) {
                        queue.push_back((next, steps - 1));
                    }
                }
            }
        }

        Some(accepted)
    }

    fn start(&self) -> Option<(usize, usize)> {
        self.find(|(_, tile)| tile == &'S')
            .map(|(coord, _)| coord.to_owned())
    }

    fn adjacent(&self, node: &(usize, usize)) -> Option<Vec<(usize, usize)>> {
        let (x, y) = node;

        Some(
            [(0, 1), (0, -1), (1, 0), (-1, 0)]
                .into_iter()
                .filter_map(|(dx, dy)| Some((x.checked_add_signed(dx)?, y.checked_add_signed(dy)?)))
                .filter(|coord| self.get(coord) != Some(&'#'))
                .collect(),
        )
    }
}

impl<const STEPS: usize> Solution for Day21<STEPS> {
    const TITLE: &'static str = "Step Counter";
    const DAY: u8 = 21;
    type Input = Garden;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Garden::from_str(input)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let accepted = input.solve(STEPS)?;

        if is_flag("--print") {
            viz::print(&input.0, &accepted);
        }

        Some(accepted.len())
    }

    fn part2(_input: &Self::Input) -> Option<Self::P2> {
        None
    }
}

mod viz {
    use std::collections::HashSet;

    pub fn print(grid: &[Vec<char>], accepted: &HashSet<(usize, usize)>) {
        for (y, row) in grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match accepted.contains(&(x, y)) {
                    true => print!("O"),
                    _ => print!("{tile}"),
                }
            }
            println!()
        }
    }
}

type Day21Actual = Day21<64>;

aoc::run!(Day21Actual);

#[allow(dead_code)]
type Day21Test = Day21<6>;

aoc::example! {
    [Day21Test]
    sample: "...........\r\n.....###.#.\r\n.###.##..#.\r\n..#.#...#..\r\n....#.#....\r\n.##..S####.\r\n.##..#...#.\r\n.......##..\r\n.##.#.####.\r\n.##..##.##.\r\n...........\r\n"
        => Some(16)
        => None
}

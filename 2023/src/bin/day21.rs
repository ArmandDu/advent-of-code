use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;

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
    fn solve(&self, start: (usize, usize), max_steps: usize) -> HashSet<(usize, usize)> {
        let mut visited = HashSet::from([start]);
        let mut accepted = HashSet::new();

        let mut queue = VecDeque::from([(start, max_steps)]);

        while let Some((current, steps)) = queue.pop_front() {
            if steps % 2 == 0 {
                accepted.insert(current);
            }

            if steps > 0 {
                for next in self.adjacent(&current) {
                    if visited.insert(next) {
                        queue.push_back((next, steps - 1));
                    }
                }
            }
        }

        accepted
    }

    fn start(&self) -> Option<(usize, usize)> {
        self.find(|(_, tile)| tile == &'S')
            .map(|(coord, _)| coord.to_owned())
    }

    fn adjacent(&self, node: &(usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = node;

        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .filter_map(|(dx, dy)| Some((x.checked_add_signed(dx)?, y.checked_add_signed(dy)?)))
            .filter(|(x, y)| *x < self.width() && *y < self.height())
            .filter(|coord| self.get(coord) != Some(&'#'))
            .collect()
    }
}

impl Garden {
    fn width(&self) -> usize {
        self.0.first().map(|row| row.len()).unwrap_or_default()
    }

    fn height(&self) -> usize {
        self.0.len()
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
        let accepted = input.solve(input.start()?, STEPS);

        if is_flag("--print") {
            viz::print(&input.0, &accepted);
        }

        Some(accepted.len())
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        const STEPS: usize = 26501365;

        let grid_size = (input.width() == input.height()).then_some(input.height())?;
        let checkerboard_width = STEPS / grid_size;

        if STEPS % grid_size != grid_size / 2 {
            return None;
        }

        let n_odd_boxes = (checkerboard_width - 1).pow(2);
        let n_even_boxes = checkerboard_width.pow(2);

        let start = input.start()?;
        let (start_x, start_y) = start;
        let cardinal_edges = [
            (0, start_y),
            (grid_size - 1, start_y),
            (start_x, 0),
            (start_x, grid_size - 1),
        ];

        let diagonal_edges = [
            (0, grid_size - 1),
            (grid_size - 1, grid_size - 1),
            (0, 0),
            (grid_size - 1, 0),
        ];

        //(start[], steps, n_boxes)
        //solve(start, steps) * n_boxes
        [
            //odd boxes
            (vec![start], grid_size + grid_size % 2 + 1, n_odd_boxes),
            //even boxes
            (vec![start], grid_size + grid_size % 2, n_even_boxes),
            //cardinal edges
            (cardinal_edges.to_vec(), grid_size - 1, 1),
            //even edges
            (
                diagonal_edges.to_vec(),
                grid_size / 2 - 1,
                checkerboard_width,
            ),
            //odd edges
            (
                diagonal_edges.to_vec(),
                grid_size * 3 / 2 - 1,
                checkerboard_width - 1,
            ),
        ]
        .into_iter()
        .map(|(edges, steps, n_boxes)| {
            edges
                .into_iter()
                .map(|start| input.solve(start, steps).len())
                .sum::<usize>()
                * n_boxes
        })
        .sum1()
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
                print!(" ")
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

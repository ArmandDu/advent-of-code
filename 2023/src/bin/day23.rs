use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;

use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;

use aoc_utils::pathfinding::Graph;

struct Trail(Vec<Vec<char>>);

struct Hike<'a>(&'a Trail, HashMap<char, Vec<(isize, isize)>>);

impl Trail {
    fn get(&self, coord: &(usize, usize)) -> Option<&char> {
        self.0.get(coord.1)?.get(coord.0)
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0.first().map(|row| row.len()).unwrap_or_default()
    }
}

impl Graph<((usize, usize), (isize, isize))> for Hike<'_> {
    fn start(&self) -> Option<((usize, usize), (isize, isize))> {
        Some(((1, 0), (0, 1))).filter(|(coord, _)| self.0.get(coord) == Some(&'.'))
    }

    fn adjacent(
        &self,
        (node, dir): &((usize, usize), (isize, isize)),
    ) -> Option<Vec<((usize, usize), (isize, isize))>> {
        self.0
            .get(node)
            .map(|tile| {
                self.1
                    .get(tile)
                    .cloned()
                    .unwrap_or(vec![(1, 0), (-1, 0), (0, -1), (0, 1)])
            })
            .map(|deltas| {
                deltas
                    .into_iter()
                    .filter(|next_dir| next_dir != &(-dir.0, -dir.1))
                    .filter_map(|(dx, dy)| {
                        Some((
                            (
                                node.0.checked_add_signed(dx)?,
                                node.1.checked_add_signed(dy)?,
                            ),
                            (dx, dy),
                        ))
                    })
                    .filter(|(next, _)| self.0.get(next) != Some(&'#'))
                    .collect_vec()
            })
    }

    fn is_target(&self, (node, _): &((usize, usize), (isize, isize))) -> bool {
        node == &(self.0.width() - 2, self.0.height() - 1)
    }
}

impl Hike<'_> {
    fn solve_longest(&self) -> Option<usize> {
        let start = self.start()?;

        let mut queue = BinaryHeap::new();
        let mut costs = HashMap::new();

        queue.push((0, start));
        costs.insert(start, 0_usize);

        let mut results = vec![];

        while let Some((cost, current)) = queue.pop() {
            if self.is_target(&current) {
                results.push(cost);
                continue;
            }

            if &cost < costs.get(&current).unwrap_or(&0) {
                continue;
            }

            for next in self.adjacent(&current)? {
                let next_cost = cost + 1;

                if &next_cost > costs.get(&next).unwrap_or(&0) {
                    costs.insert(next, next_cost);
                    queue.push((next_cost, next));
                }
            }
        }

        results.iter().cloned().max()
    }
}

impl FromStr for Trail {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Trail(
            s.lines().map(|line| line.chars().collect()).collect(),
        ))
    }
}

struct Day23;

impl Solution for Day23 {
    const TITLE: &'static str = "A Long Walk";
    const DAY: u8 = 23;
    type Input = Trail;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Trail::from_str(input)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let slopes = [
            ('>', vec![(1, 0)]),
            ('<', vec![(-1, 0)]),
            ('^', vec![(0, -1)]),
            ('v', vec![(0, 1)]),
        ]
        .into_iter()
        .collect();
        let hike = Hike(input, slopes);

        hike.solve_longest()
    }

    fn part2(_input: &Self::Input) -> Option<Self::P2> {
        None
    }
}

aoc::run!(Day23);

aoc::example! {
    [Day23]
    sample: "#.#####################\r\n#.......#########...###\r\n#######.#########.#.###\r\n###.....#.>.>.###.#.###\r\n###v#####.#v#.###.#.###\r\n###.>...#.#.#.....#...#\r\n###v###.#.#.#########.#\r\n###...#.#.#.......#...#\r\n#####.#.#.#######.#.###\r\n#.....#.#.#.......#...#\r\n#.#####.#.#.#########v#\r\n#.#...#...#...###...>.#\r\n#.#.#v#######v###.###v#\r\n#...#.>.#...>.>.#.###.#\r\n#####v#.#.###v#.#.###.#\r\n#.....#...#...#.#.#...#\r\n#.#########.###.#.#.###\r\n#...###...#...#...#.###\r\n###.###.#.###v#####v###\r\n#...#...#.#.>.>.#.>.###\r\n#.###.###.#.###.#.#v###\r\n#.....###...###...#...#\r\n#####################.#\r\n"
        => Some(94)
        => None
}

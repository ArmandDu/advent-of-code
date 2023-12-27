use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

use aoc::solution::SolutionError;
use aoc::Solution;

use aoc_utils::is_flag;
use aoc_utils::pathfinding::Graph;

type Coord = (usize, usize);

struct Trail(Vec<Vec<char>>);

struct Hike(Coord, Coord, HashMap<Coord, HashMap<Coord, usize>>);

impl Trail {
    fn get(&self, coord: &Coord) -> Option<&char> {
        self.0.get(coord.1)?.get(coord.0)
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0.first().map(|row| row.len()).unwrap_or_default()
    }

    fn adjacent(&self, current: &Coord, deltas: &[(isize, isize)]) -> Vec<(usize, usize)> {
        deltas
            .iter()
            .cloned()
            .filter_map(|(dx, dy)| {
                Some((
                    current.0.checked_add_signed(dx)?,
                    current.1.checked_add_signed(dy)?,
                ))
            })
            .filter(|(x, y)| *x < self.width() && *y < self.height())
            .filter(|next| self.get(next) != Some(&'#'))
            .collect()
    }
}

impl Hike {
    fn new(trail: &Trail, dirs: HashMap<char, Vec<(isize, isize)>>) -> Self {
        const ALL_DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];
        let start = (1, 0);
        let target = (trail.width() - 2, trail.height() - 1);

        let nodes = {
            let mut intersections = vec![start, target];

            for (y, row) in trail.0.iter().enumerate() {
                for (x, tile) in row.iter().enumerate() {
                    if tile != &'#' && trail.adjacent(&(x, y), &ALL_DIRS).len() > 2 {
                        intersections.push((x, y));
                    }
                }
            }

            intersections
        };

        let mut graph: HashMap<_, HashMap<_, _>> = HashMap::new();

        for node in nodes.iter() {
            let node = *node;
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();

            queue.push_back((node, 0));
            visited.insert(node);

            while let Some((current, dist)) = queue.pop_front() {
                if dist > 0 && nodes.contains(&current) {
                    *graph.entry(node).or_default().entry(current).or_default() = dist;
                    continue;
                }

                let tile = trail.get(&current).unwrap_or(&'.');

                for next in trail.adjacent(&current, dirs.get(tile).unwrap_or(&ALL_DIRS.to_vec())) {
                    if visited.insert(next) {
                        queue.push_back((next, dist + 1))
                    }
                }
            }
        }

        Self(start, target, graph)
    }
}

impl Graph<Coord> for Hike {
    fn start(&self) -> Option<Coord> {
        Some(self.0)
    }

    fn adjacent(&self, current: &Coord) -> Option<Vec<Coord>> {
        self.2
            .get(current)
            .map(|children| children.keys().cloned().collect())
    }

    fn is_target(&self, node: &Coord) -> bool {
        &self.1 == node
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

impl Hike {
    fn longest_path(&self) -> Option<usize> {
        let start = self.0;
        let end = self.1;
        let graph = &self.2;

        let mut queue = vec![(start, 0, HashSet::new())];
        let mut distances = vec![];

        while let Some((current, dist, visited)) = queue.pop() {
            if current == end {
                distances.push(dist);
                continue;
            }

            if let Some(nodes) = graph.get(&current) {
                let mut next_visited = visited.to_owned();

                next_visited.insert(current);
                for (next, cost) in nodes {
                    if !visited.contains(next) {
                        queue.push((*next, dist + cost, next_visited.to_owned()))
                    }
                }
            }
        }

        distances.into_iter().max()
    }
}

mod vis {
    use crate::{Hike, Trail};

    impl Hike {
        pub fn dbg(&self, trail: &Trail) {
            for (y, row) in trail.0.iter().enumerate() {
                for (x, tile) in row.iter().enumerate() {
                    if self.2.contains_key(&(x, y)) {
                        print!(" X ");
                    } else {
                        match tile {
                            '.' => print!("   "),
                            '#' => print!("███"),
                            t => print!(" {t} "),
                        }
                    }
                }
                println!(" {y}")
            }
        }
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
        let slopes: HashMap<_, _> = [
            ('>', vec![(1, 0)]),
            ('<', vec![(-1, 0)]),
            ('^', vec![(0, -1)]),
            ('v', vec![(0, 1)]),
        ]
        .into_iter()
        .collect();

        let hike = Hike::new(input, slopes.to_owned());

        is_flag("--print").then(|| {
            hike.dbg(input);
        });

        hike.longest_path()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let hike = Hike::new(input, Default::default());

        hike.longest_path()
    }
}

aoc::run!(Day23);

aoc::example! {
    [Day23]
    sample: "#.#####################\r\n#.......#########...###\r\n#######.#########.#.###\r\n###.....#.>.>.###.#.###\r\n###v#####.#v#.###.#.###\r\n###.>...#.#.#.....#...#\r\n###v###.#.#.#########.#\r\n###...#.#.#.......#...#\r\n#####.#.#.#######.#.###\r\n#.....#.#.#.......#...#\r\n#.#####.#.#.#########v#\r\n#.#...#...#...###...>.#\r\n#.#.#v#######v###.###v#\r\n#...#.>.#...>.>.#.###.#\r\n#####v#.#.###v#.#.###.#\r\n#.....#...#...#.#.#...#\r\n#.#########.###.#.#.###\r\n#...###...#...#...#.###\r\n###.###.#.###v#####v###\r\n#...#...#.#.>.>.#.>.###\r\n#.###.###.#.###.#.#v###\r\n#.....###...###...#...#\r\n#####################.#\r\n"
        => Some(94)
        => Some(154)
}

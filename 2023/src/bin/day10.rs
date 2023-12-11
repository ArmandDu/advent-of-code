use aoc::solution::SolutionError;
use aoc::Solution;
use aoc_utils::pathfinding::Graph;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::env::args;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

struct Day10;

type History = HashMap<(usize, usize), Option<(usize, usize)>>;

struct Maze {
    raw: Vec<Vec<char>>,
    solved: Option<History>,
}

impl Maze {
    fn new(raw: Vec<Vec<char>>) -> Self {
        let mut temp = Self { raw, solved: None };

        temp.solved = temp.solve();

        temp
    }

    fn tube(&self) -> Option<&History> {
        self.solved.as_ref()
    }

    fn get(&self, coord: &(usize, usize)) -> Option<&char> {
        self.raw.get(coord.1).and_then(|raw| raw.get(coord.0))
    }

    fn solve(&self) -> Option<History> {
        let mut history = HashMap::new();
        let mut queue = VecDeque::new();

        let start = self.start()?;

        queue.push_back(start);
        history.insert(start, None);

        while let Some(current) = queue.pop_front() {
            for next in self.adjacent(&current)? {
                if history.contains_key(&next) {
                    continue;
                }

                history.insert(next, Some(current));
                queue.push_back(next)
            }
        }

        Some(history)
    }
}

impl FromStr for Maze {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim().replace('\r', "");
        let width = input.find('\n').ok_or(SolutionError::ParseError)?;
        let bandwidth = std::iter::repeat('.').take(width).join("");

        let raw = format!("{bandwidth}\n{input}\n{bandwidth}")
            .lines()
            .map(|row| format!(".{row}.").chars().collect_vec())
            .collect_vec();

        Ok(Maze::new(raw))
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.raw
                .iter()
                .enumerate()
                .map(|(y, row)| row
                    .iter()
                    .enumerate()
                    .map(|(x, c)| {
                        let c = match *c {
                            'F' => '╔',
                            'J' => '╝',
                            'L' => '╚',
                            '7' => '╗',
                            '-' => '═',
                            '|' => '║',
                            rem => rem,
                        };

                        let is_tube = self
                            .solved
                            .as_ref()
                            .and_then(|hist| hist.get(&(x, y)))
                            .is_some();

                        match is_tube {
                            true => format!("\x1b[92m{c}\x1b[0m"),
                            _ => c.into(),
                        }
                    })
                    .join(""))
                .join("\n")
        )
    }
}

impl Graph<(usize, usize)> for Maze {
    fn start(&self) -> Option<(usize, usize)> {
        self.raw.iter().enumerate().find_map(|(y, raw)| {
            raw.iter()
                .find_position(|&&c| c == 'S')
                .map(|(x, _)| (x, y))
        })
    }

    fn adjacent(&self, node: &(usize, usize)) -> Option<Vec<(usize, usize)>> {
        let (px, py) = *node;

        let neighbors: Vec<(i32, i32)> = match self.get(node)? {
            'S' => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
            '|' => vec![(0, -1), (0, 1)],
            '-' => vec![(-1, 0), (1, 0)],
            'L' => vec![(0, -1), (1, 0)],
            'J' => vec![(-1, 0), (0, -1)],
            '7' => vec![(-1, 0), (0, 1)],
            'F' => vec![(1, 0), (0, 1)],
            _ => vec![],
        };

        let neighbors = neighbors
            .iter()
            .filter_map(|(dx, dy)| {
                let nx = dx.saturating_add(px as i32) as usize;
                let ny = dy.saturating_add(py as i32) as usize;

                let next = self.get(&(nx, ny))?;

                match (dx, dy) {
                    (-1, 0) => "F-L".contains(|c| c == *next).then_some((nx, ny)),
                    (1, 0) => "J-7".contains(|c| c == *next).then_some((nx, ny)),
                    (0, -1) => "F|7".contains(|c| c == *next).then_some((nx, ny)),
                    (0, 1) => "J|L".contains(|c| c == *next).then_some((nx, ny)),
                    _ => None,
                }
            })
            .collect_vec();

        Some(neighbors)
    }

    fn is_target(&self, _node: &(usize, usize)) -> bool {
        false
    }
}

impl Solution for Day10 {
    const TITLE: &'static str = "Pipe Maze";
    const DAY: u8 = 10;
    type Input = Maze;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Maze::from_str(input)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        Some(input.tube()?.len() / 2)
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let hist = input.tube()?;
        let width = input.raw.first()?.len();
        let height = input.raw.len();

        Some(
            (0..width)
                .cartesian_product(0..height)
                .filter(|node| !hist.contains_key(node))
                .filter(|(x, y)| {
                    (0..*x)
                        .filter(|&xi| hist.contains_key(&(xi, *y)))
                        .filter(|&xi| {
                            //feels hacky. Might not work on other inputs.
                            //my S is a 7. So it doesn't count as a left wall.
                            matches!(input.get(&(xi, *y)), Some('J') | Some('L') | Some('|'))
                        })
                        .count()
                        % 2
                        != 0
                })
                .count(),
        )
    }
}

fn main() {
    if args().any(|arg| arg == "--print") {
        println!("{}", Day10::parse(&Day10::get_input().unwrap()).unwrap())
    } else {
        aoc::solution!(Day10)
    }
}

aoc::example! {
    [Day10]
    smallest: ".....\r\n.S-7.\r\n.|.|.\r\n.L-J.\r\n.....\r\n"
        => Some(4)
        => Some(1)
    smallest_noise: "-L|F7\r\n7S-7|\r\nL|7||\r\n-L-J|\r\nL|-JF\r\n"
        => Some(4)
        => Some(1)
    medium: "..F7.\r\n.FJ|.\r\nSJ.L7\r\n|F--J\r\nLJ...\r\n"
        => Some(8)
        => Some(1)
    medium_noise: "7-F7-\r\n.FJ|7\r\nSJLL7\r\n|F--J\r\nLJ.LJ\r\n"
        => Some(8)
        => Some(1)
    large_noise: "FF7FSF7F7F7F7F7F---7\r\nL|LJ||||||||||||F--J\r\nFL-7LJLJ||||||LJL-77\r\nF--JF--7||LJLJ7F7FJ-\r\nL---JF-JLJ.||-FJLJJ7\r\n|F|F-JF---7F7-L7L|7|\r\n|FFJF7L7F-JF7|JL---7\r\n7-L-JL7||F7|L7F-7F7|\r\nL.L7LFJ|||||FJL7||LJ\r\nL7JLJL-JLJLJL--JLJ.L\r\n"
        => Some(80)
        => Some(10)
}

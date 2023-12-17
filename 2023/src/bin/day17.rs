use aoc::solution::SolutionError;
use aoc::Solution;
use aoc_utils::pathfinding::Graph;
use std::str::FromStr;

struct Day17;

struct Crucible(i8, i8);

struct City(Vec<Vec<u32>>);

struct Maze<'a>(&'a City, Crucible);

impl Maze<'_> {
    fn min_blocks(&self) -> i8 {
        self.1 .0
    }

    fn max_blocks(&self) -> i8 {
        self.1 .1
    }

    fn get_cost_fn(
        &self,
    ) -> impl Fn(
        &((usize, usize), (isize, isize), i8),
        &((usize, usize), (isize, isize), i8),
    ) -> Option<i32>
           + '_ {
        let city = self.0;

        |_left, right| city.get(&right.0).map(|&c| c as i32)
    }

    fn solve(&self) -> Option<i32> {
        aoc_utils::pathfinding::dijkstra::solve(self, self.get_cost_fn()).map(|(count, _)| count)
    }
}

impl City {
    fn width(&self) -> usize {
        self.0.first().map(|row| row.len()).unwrap_or_default()
    }

    fn height(&self) -> usize {
        self.0.len()
    }
    fn get(&self, coord: &(usize, usize)) -> Option<&u32> {
        self.0.get(coord.1)?.get(coord.0)
    }
}

impl FromStr for City {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| {
                line.chars()
                    .map(|tile| tile.to_digit(10).ok_or(SolutionError::ParseError))
                    .collect()
            })
            .collect::<Result<_, _>>()
            .map(City)
    }
}

impl Graph<((usize, usize), (isize, isize), i8)> for Maze<'_> {
    fn start(&self) -> Option<((usize, usize), (isize, isize), i8)> {
        Some(((0, 0), (0, 0), 0))
    }

    fn adjacent(
        &self,
        node: &((usize, usize), (isize, isize), i8),
    ) -> Option<Vec<((usize, usize), (isize, isize), i8)>> {
        let ((x, y), dir, count) = *node;

        Some(
            [(0, 1), (0, -1), (1, 0), (-1, 0)]
                .into_iter()
                .filter(|(dx, dy)| (-dx, -dy) != dir)
                .filter(|&next_dir| !(next_dir == dir && count + 1 >= self.max_blocks()))
                .filter(|&next_dir| {
                    !(dir != (0, 0) && next_dir != dir && count + 1 < self.min_blocks())
                })
                .filter_map(|(dx, dy)| {
                    Some((
                        (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?),
                        (dx, dy),
                        if dir == (dx, dy) { count + 1 } else { 0 },
                    ))
                })
                .collect(),
        )
    }

    fn is_target(&self, node: &((usize, usize), (isize, isize), i8)) -> bool {
        let ((x, y), _, _) = node;

        *x == self.0.width() - 1 && *y == self.0.height() - 1
    }
}

impl Solution for Day17 {
    const TITLE: &'static str = "Clumsy Crucible";
    const DAY: u8 = 17;
    type Input = City;
    type P1 = i32;
    type P2 = i32;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        City::from_str(input)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let maze = Maze(input, Crucible(0, 3));

        maze.solve()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let maze = Maze(input, Crucible(4, 10));

        maze.solve()
    }
}

aoc::run!(Day17);

aoc::example! {
    [Day17]
    sample: "2413432311323\r\n3215453535623\r\n3255245654254\r\n3446585845452\r\n4546657867536\r\n1438598798454\r\n4457876987766\r\n3637877979653\r\n4654967986887\r\n4564679986453\r\n1224686865563\r\n2546548887735\r\n4322674655533\r\n"
        => Some(102)
        => Some(94)
}

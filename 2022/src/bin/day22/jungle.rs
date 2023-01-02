use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use aoc::solution::SolutionError;
use itertools::Itertools;

use aoc_utils::dijkstra::Boundaries;

use crate::dir::Dir;
use crate::Tile;

pub trait Navigate {
    fn get(&self, current: (usize, usize), dir: Dir) -> Option<&Tile>;
    fn pos(&self, current: (usize, usize), dir: Dir) -> Option<((usize, usize), Dir)>;
}

#[derive(Debug, Clone)]
pub struct Jungle {
    grid: HashMap<(usize, usize), Tile>,
    pub boundaries: Boundaries,
}

impl Jungle {
    pub fn get(&self, coord: &(usize, usize)) -> Option<&Tile> {
        self.grid.get(coord)
    }

    pub fn top_left(&self) -> Option<(usize, usize)> {
        let step = self.boundaries.width.max(self.boundaries.height) / 4;

        (0..self.boundaries.width)
            .step_by(step)
            .find(|x| self.grid.contains_key(&(*x, 0)))
            .map(|x| (x, 0))
    }
}

impl FromStr for Jungle {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: HashMap<_, _> = s
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(move |(x, char)| match char {
                        '.' => Some(((x, y), Tile::Open)),
                        '#' => Some(((x, y), Tile::Solid)),
                        _ => None,
                    })
            })
            .collect();

        let (x_max, y_max) = grid
            .iter()
            .fold((usize::MIN, usize::MIN), |(x_max, y_max), (coord, _)| {
                (x_max.max(coord.0 + 1), y_max.max(coord.1 + 1))
            });

        Ok(Self {
            grid,
            boundaries: Boundaries {
                x: 0,
                y: 0,
                width: x_max,
                height: y_max,
            },
        })
    }
}

pub struct Render<'a>(&'a Jungle, &'a HashMap<(usize, usize), Dir>);

impl<'a> Render<'a> {
    pub fn new(jungle: &'a Jungle, path: &'a HashMap<(usize, usize), Dir>) -> Render<'a> {
        Self(jungle, path)
    }
}

impl Display for Render<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self(jungle, path) = self;

        write!(
            f,
            "{}",
            (jungle.boundaries.y..jungle.boundaries.height)
                .map(|y| {
                    (jungle.boundaries.x..jungle.boundaries.width)
                        .map(|x| match path.get(&(x, y)) {
                            Some(&dir) => match dir {
                                Dir::Right => ">",
                                Dir::Down => "v",
                                Dir::Left => "<",
                                Dir::Up => "^",
                            },
                            _ => match jungle.get(&(x, y)) {
                                Some(Tile::Solid) => "#",
                                Some(Tile::Open) => ".",
                                None => " ",
                            },
                        })
                        .join("")
                })
                .join("\n"),
        )
    }
}

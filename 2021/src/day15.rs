use crate::utils::neighbors;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::collections::HashMap;
use year2021::Solution;

pub struct Day15;

impl Solution<usize, usize> for Day15 {
    const DAY: u32 = 15;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "";
    type Input = (Vec<Vec<i32>>, usize, usize);

    fn part1(input: &Self::Input) -> Option<usize> {
        let (map, width, height) = input;

        Self::get_cost(map, width, height)
    }

    fn part2(input: &Self::Input) -> Option<usize> {
        let (map, width, height) = input;

        let actual_map: Vec<_> = (0..(5 * height))
            .map(|y| {
                (0..(5 * width))
                    .map(|x| {
                        let original = map[y % height][x % width];

                        match original + (x / width + y / height) as i32 {
                            x if x > 9 => x - 9,
                            x => x,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        Self::get_cost(&actual_map, &(*width * 5), &(*height * 5))
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        let map: Vec<Vec<i32>> = input
            .trim()
            .lines()
            .map(|line| line.chars().map(|c| (c as u8 - b'0') as i32).collect())
            .collect();
        let height = map.len();
        let width = input.find("\n").expect("Invalid input");

        Ok((map, width, height))
    }
}

impl Day15 {
    fn get_cost(map: &Vec<Vec<i32>>, width: &usize, height: &usize) -> Option<usize> {
        let solver = AStar::new(map.to_owned(), *width, *height);
        let start = (0, 0);
        let end = (*width - 1, *height - 1);

        solver
            .solve(&start, &end, Self::manhattan_dist)
            .and_then(|solve| Some(solver.get_path(&solve, &start, &end)))
            .and_then(|path| Some(path.iter().map(|&(_, cost)| cost as usize).sum()))
    }

    fn manhattan_dist(a: &(usize, usize), b: &(usize, usize)) -> i32 {
        let (ax, ay) = *a;
        let (bx, by) = *b;

        (ax as i32 - bx as i32).abs() + (ay as i32 - by as i32).abs()
    }
}

#[derive(Debug)]
struct AStar {
    map: HashMap<(usize, usize), i32>,
    width: usize,
    height: usize,
}

impl AStar {
    pub fn new(map: Vec<Vec<i32>>, width: usize, height: usize) -> Self {
        Self {
            map: (0..height)
                .cartesian_product(0..width)
                .map(|(y, x)| ((x, y), map[y][x]))
                .collect(),
            width,
            height,
        }
    }

    pub fn get_path(
        &self,
        solve: &HashMap<(usize, usize), Option<(usize, usize)>>,
        start: &(usize, usize),
        destination: &(usize, usize),
    ) -> Vec<((usize, usize), i32)> {
        let mut path = vec![];
        let mut current = destination.clone();

        while current != *start {
            path.push((current, *self.map.get(&current).unwrap()));

            current = solve.get(&current).unwrap().unwrap();
        }

        path.into_iter().rev().collect()
    }

    pub fn solve(
        &self,
        start: &(usize, usize),
        destination: &(usize, usize),
        h: impl Fn(&(usize, usize), &(usize, usize)) -> i32,
    ) -> Option<HashMap<(usize, usize), Option<(usize, usize)>>> {
        let mut frontier = PriorityQueue::new();
        let mut came_from: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
        let mut cost_so_far: HashMap<(usize, usize), i32> = HashMap::new();

        frontier.push(*start, i32::MAX);
        came_from.insert(*start, None);
        cost_so_far.insert(*start, 0);

        while !frontier.is_empty() {
            let (current, _) = frontier.pop().unwrap();

            if current == *destination {
                return Some(came_from);
            }

            neighbors(self.width, self.height, current.0, current.1)
                .iter()
                .for_each(|&next| {
                    let new_cost =
                        cost_so_far.get(&current).unwrap_or(&0) + self.map.get(&next).unwrap_or(&0);

                    if new_cost < *cost_so_far.get(&next).unwrap_or(&i32::MAX) {
                        let priority = new_cost + h(&destination, &next);

                        cost_so_far.insert(next, new_cost);
                        frontier.push(next, i32::MAX - priority);
                        came_from.insert(next, Some(current));
                    }
                })
        }

        None
    }
}

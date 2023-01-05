pub fn lines_to_owned(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_owned()).collect()
}

pub fn neighbors(width: usize, height: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
    [
        (y > 0).then(|| (x, y - 1)),
        (x > 0).then(|| (x - 1, y)),
        (x + 1 < width).then(|| (x + 1, y)),
        (y + 1 < height).then(|| (x, y + 1)),
    ]
    .iter()
    .filter_map(|&c| c)
    .collect()
}

pub fn lcm(x: usize, y: usize) -> Option<usize> {
    let max = x.max(y);

    (max..).find(|&max| max % x == 0 && max % y == 0)
}

pub fn index(value: isize, min: isize, max: isize) -> usize {
    let size_y = max.abs_diff(min) as isize;

    (min + (max + (value % size_y)) % size_y) as usize
}

// https://doc.rust-lang.org/std/collections/binary_heap/index.html
pub mod dijkstra {
    use super::neighbors;
    use std::collections::BinaryHeap;

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct Coord {
        pub x: usize,
        pub y: usize,
    }

    #[derive(Debug, Clone)]
    pub struct Boundaries {
        pub x: usize,
        pub y: usize,
        pub width: usize,
        pub height: usize,
    }

    pub fn solve(
        start: &Coord,
        boundaries: &Boundaries,
        get_cost: impl Fn(&Coord, &Coord) -> Option<i32>,
        is_target: impl Fn(&Coord) -> bool,
    ) -> Option<(Vec<Coord>, i32)> {
        let mut dist: Vec<Vec<_>> = (0..boundaries.height)
            .map(|_| (0..boundaries.width).map(|_| i32::MAX).collect())
            .collect();
        let mut heap = BinaryHeap::new();

        heap.push((0, start.x, start.y));
        dist[start.y][start.x] = 0;

        while let Some((cost, x, y)) = heap.pop() {
            let current = Coord { x, y };
            if is_target(&current) {
                return Some((get_path(&dist, start, &current), -cost));
            }

            if cost > dist[current.y][current.x] {
                continue;
            }

            for (x, y) in neighbors(boundaries.width, boundaries.height, current.x, current.y) {
                let neighbor = Coord { x, y };
                let next_cost = match get_cost(&current, &neighbor) {
                    Some(new_cost) => -cost + new_cost,
                    None => continue,
                };

                if next_cost < dist[neighbor.y][neighbor.x] {
                    heap.push((-next_cost, neighbor.x, neighbor.y));
                    dist[neighbor.y][neighbor.x] = next_cost;
                }
            }
        }
        None
    }

    fn get_path(dist: &[Vec<i32>], start: &Coord, target: &Coord) -> Vec<Coord> {
        let mut path = vec![];

        let mut current = *target;

        while current.x != start.x && current.y != start.y {
            path.push(current);

            current = match neighbors(dist[0].len(), dist.len(), current.x, current.y)
                .iter()
                .min_by(|&&(xa, ya), &&(xb, yb)| dist[ya][xa].cmp(&dist[yb][xb]))
            {
                Some(&(x, y)) => Coord { x, y },
                _ => panic!(),
            }
        }
        path.push(*start);
        path.into_iter().rev().collect()
    }
}

pub mod pathfinding {
    use std::collections::{HashMap, VecDeque};
    use std::hash::Hash;

    pub fn bfs<N, IT>(
        start: &N,
        adjacent: impl Fn(&N) -> IT,
        is_target: impl Fn(&N) -> bool,
    ) -> Option<Vec<N>>
    where
        N: Copy + Hash + Eq,
        IT: Iterator<Item = N>,
    {
        let mut history = HashMap::new();
        let mut queue = VecDeque::new();

        queue.push_back(*start);
        history.insert(*start, None);

        while let Some(current) = queue.pop_front() {
            if is_target(&current) {
                return get_path(&history, start, current);
            }
            for next in adjacent(&current) {
                if history.contains_key(&next) {
                    continue;
                }

                history.insert(next, Some(current));
                queue.push_back(next);
            }
        }

        None
    }

    fn get_path<N>(history: &HashMap<N, Option<N>>, start: &N, mut target: N) -> Option<Vec<N>>
    where
        N: Copy + Hash + Eq,
    {
        let mut path = vec![target];

        while let Some(Some(parent)) = history.get(&target) {
            path.push(*parent);
            target = *parent;

            if start == parent {
                return Some(path.into_iter().rev().collect());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::pathfinding::*;

    #[test]
    fn bfs_simple() {
        let start = 'A';
        let goal = 'Z';

        let path = bfs(
            &start,
            |&point| [(point as u8 + 1) as char].into_iter(),
            |&n| n == goal,
        );

        assert_eq!(path.expect("path not found").len(), 26);
    }
}

pub mod pathfinding;

pub fn is_flag(flag: &str) -> bool {
    std::env::args().any(|arg| arg.as_str() == flag)
}

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

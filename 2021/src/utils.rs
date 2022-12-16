pub mod a_star {
    use aoc_utils::neighbors;
    use std::collections::{BinaryHeap, HashMap};

    type Coord = (usize, usize);
    type Boundaries = (usize, usize);

    pub fn solve(
        start: &Coord,
        target: &Coord,
        boundaries: &Boundaries,
        get_cost: impl Fn(&Coord, &Coord) -> Option<i32>,
        heuristic: impl Fn(&Coord, &Coord) -> i32,
    ) -> Option<(Vec<Coord>, i32)> {
        let mut frontier = BinaryHeap::new();
        let mut came_from: HashMap<Coord, Option<Coord>> = HashMap::new();
        let mut cost_so_far: HashMap<Coord, i32> = HashMap::new();

        frontier.push((i32::MAX, *start));
        cost_so_far.insert(*start, 0);

        while let Some((cost, current)) = frontier.pop() {
            if current == *target {
                return Some((get_path(&came_from, start, target), i32::MAX - cost));
            }

            neighbors(boundaries.0, boundaries.1, current.0, current.1)
                .iter()
                .for_each(|&next| {
                    let new_cost = cost_so_far.get(&current).unwrap_or(&0)
                        + get_cost(&current, &next).unwrap_or(0);

                    if new_cost < *cost_so_far.get(&next).unwrap_or(&i32::MAX) {
                        let priority = new_cost + heuristic(target, &next);

                        cost_so_far.insert(next, new_cost);
                        frontier.push((i32::MAX - priority, next));
                        came_from.insert(next, Some(current));
                    }
                })
        }

        None
    }

    fn get_path(
        solve: &HashMap<(usize, usize), Option<(usize, usize)>>,
        start: &(usize, usize),
        destination: &(usize, usize),
    ) -> Vec<(usize, usize)> {
        let mut path = vec![];
        let mut current = *destination;

        while current != *start && !solve.is_empty() {
            path.push(current);

            current = solve.get(&current).unwrap().unwrap();
        }

        path.into_iter().rev().collect()
    }
}

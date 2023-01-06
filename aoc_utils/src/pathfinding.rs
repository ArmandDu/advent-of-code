use std::collections::HashMap;
use std::hash::Hash;

pub trait Graph<N> {
    fn start(&self) -> Option<N>;
    fn adjacent(&self, node: &N) -> Option<Vec<N>>;
    fn is_target(&self, node: &N) -> bool;

    fn estimated_size(&self) -> usize {
        0
    }
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

pub mod dijkstra {
    use crate::pathfinding::get_path;
    use std::collections::{BinaryHeap, HashMap};
    use std::fmt::Debug;
    use std::hash::Hash;

    pub fn solve<G, N>(
        graph: &G,
        get_cost: impl Fn(&N, &N) -> Option<i32>,
    ) -> Option<(i32, impl Fn() -> Option<Vec<N>>)>
    where
        N: Copy + Hash + Eq + Ord + Debug,
        G: super::Graph<N>,
    {
        let start = graph.start()?;
        let mut queue = BinaryHeap::new();
        let mut cost_lookup = HashMap::with_capacity(graph.estimated_size());
        let mut history = HashMap::new();

        queue.push((0, start));
        cost_lookup.insert(start, 0);
        history.insert(start, None);

        while let Some((cost, current)) = queue.pop() {
            if graph.is_target(&current) {
                return Some((-cost, move || get_path(&history, &start, current)));
            }

            if &cost > cost_lookup.get(&current).unwrap_or(&i32::MAX) {
                continue;
            }

            for next in graph.adjacent(&current)? {
                let next_cost = match get_cost(&current, &next) {
                    None => continue,
                    Some(new_cost) => -cost + new_cost,
                };

                if &next_cost < cost_lookup.get(&next).unwrap_or(&i32::MAX) {
                    queue.push((-next_cost, next));
                    cost_lookup.insert(next, next_cost);
                    history.insert(next, Some(current));
                }
            }
        }

        None
    }
}

pub mod bfs {
    use std::collections::{HashMap, VecDeque};
    use std::hash::Hash;

    use super::{get_path, Graph};

    pub fn solve<G, N>(graph: &G) -> Option<(N, impl Fn() -> Option<Vec<N>>)>
    where
        N: Copy + Hash + Eq,
        G: Graph<N>,
    {
        let mut history = HashMap::new();
        let mut queue = VecDeque::new();

        let start = graph.start()?;

        queue.push_back(start);
        history.insert(start, None);

        while let Some(current) = queue.pop_front() {
            if graph.is_target(&current) {
                return Some((current, move || get_path(&history, &start, current)));
            }
            for next in graph.adjacent(&current)? {
                if history.contains_key(&next) {
                    continue;
                }

                history.insert(next, Some(current));
                queue.push_back(next);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::pathfinding::*;

    struct Alphabet;

    impl Graph<char> for Alphabet {
        fn start(&self) -> Option<char> {
            Some('A')
        }
        fn adjacent(&self, node: &char) -> Option<Vec<char>> {
            Some(vec![(*node as u8 + 1) as char])
        }
        fn is_target(&self, node: &char) -> bool {
            node == &'Z'
        }
    }

    #[test]
    fn bfs_simple() {
        let (last, get_path) = bfs::solve(&Alphabet).expect("path not found");

        assert_eq!(26, get_path().expect("path not found").len());
        assert_eq!('Z', last);
    }

    #[test]
    fn djikstra_simple() {
        let (cost, get_path) = dijkstra::solve(&Alphabet, |_, _| Some(1)).expect("path not found");

        assert_eq!(25, cost);
        assert_eq!(26, get_path().expect("path not found").len());
    }
}

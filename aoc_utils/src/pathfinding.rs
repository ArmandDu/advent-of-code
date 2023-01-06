pub mod bfs {
    use std::collections::{HashMap, VecDeque};
    use std::hash::Hash;

    pub trait Graph<N: Copy + Hash + Eq> {
        fn start(&self) -> Option<N>;
        fn adjacent(&self, node: &N) -> Option<Vec<N>>;
        fn is_target(&self, node: &N) -> bool;
    }

    pub fn solve<G, N>(graph: &G) -> Option<Vec<N>>
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
                return get_path(&history, &start, current);
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

    struct Alphabet;

    impl bfs::Graph<char> for Alphabet {
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
        let path = bfs::solve(&Alphabet);

        assert_eq!(path.expect("path not found").len(), 26);
    }
}

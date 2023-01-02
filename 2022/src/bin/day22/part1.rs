use itertools::Itertools;

use crate::dir::Dir;
use crate::jungle::{Jungle, Navigate};
use crate::{index, Tile};

pub struct SphereJungle<'a> {
    jungle: &'a Jungle,
    x_ranges: Vec<(usize, usize)>,
    y_ranges: Vec<(usize, usize)>,
}

impl<'a> SphereJungle<'a> {
    pub fn new(jungle: &'a Jungle) -> Self {
        let step = jungle.boundaries.width.max(jungle.boundaries.height) / 4;

        let x_ranges = (0..jungle.boundaries.height)
            .filter_map(|y| {
                (0..jungle.boundaries.width)
                    .step_by(step)
                    .filter(|x| jungle.get(&(*x, y)).is_some())
                    .minmax()
                    .into_option()
                    .map(|(min, max)| (min, max + step))
            })
            .collect_vec();

        let y_ranges = (0..jungle.boundaries.width)
            .filter_map(|x| {
                (0..jungle.boundaries.height)
                    .step_by(step)
                    .filter(|y| jungle.get(&(x, *y)).is_some())
                    .minmax()
                    .into_option()
                    .map(|(min, max)| (min, max + step))
            })
            .collect_vec();

        Self {
            jungle,
            x_ranges,
            y_ranges,
        }
    }
}

impl<'a> Navigate for SphereJungle<'a> {
    fn get(&self, current: (usize, usize), dir: Dir) -> Option<&Tile> {
        let (next, _) = self.pos(current, dir)?;

        self.jungle.get(&next)
    }

    fn pos(&self, (x, y): (usize, usize), dir: Dir) -> Option<((usize, usize), Dir)> {
        let (dx, dy) = dir.forward();
        let next_x = x as isize + dx;
        let next_y = y as isize + dy;

        let (min_y, max_y) = self.y_ranges.get(x)?;
        let rel_y = index(next_y, *min_y as isize, *max_y as isize);

        let (min_x, max_x) = self.x_ranges.get(rel_y)?;
        let rel_x = index(next_x, *min_x as isize, *max_x as isize);

        Some(((rel_x, rel_y), dir))
    }
}

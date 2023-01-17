use crate::factory::Factory;
use crate::material::{MatCount, Material, RobotCost};
use derive_more::From;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;

#[derive(Debug, From)]
pub struct Blueprint {
    id: usize,
    robots: Vec<RobotCost>,
    max_requirements: HashMap<Material, MatCount>,
}

impl Blueprint {
    pub fn new(id: usize, robots: Vec<RobotCost>) -> Self {
        Self {
            id,
            max_requirements: Material::iter()
                .map(|mat| {
                    let cost = robots
                        .iter()
                        .filter_map(|c| c.cost.get(&mat))
                        .map(|mat_cost| mat_cost.into_inner())
                        .max()
                        .unwrap_or(usize::MAX);

                    (mat, cost.into())
                })
                .collect(),
            robots,
        }
    }

    pub fn get(&self, material: &Material) -> Option<&RobotCost> {
        self.robots.iter().find(|r| r.material == *material)
    }

    pub fn max_requirement(&self, material: &Material) -> Option<&MatCount> {
        self.max_requirements.get(material)
    }
}

impl Blueprint {
    pub fn solve(&self, budget: usize) -> usize {
        let mut best = 0;

        let mut queue = BinaryHeap::new();

        queue.push((Factory::new(self), budget));

        while let Some((factory, remaining_time)) = queue.pop() {
            for (material, time_to_build) in Material::iter()
                .filter(|mat| factory.should_construct(mat))
                .filter_map(|mat| Some((mat, factory.get_time_to_build(&mat)? + 1)))
                .filter(|(_, time)| time < &remaining_time)
            {
                let mut next_factory = factory.clone();
                let remaining_time = remaining_time.saturating_sub(time_to_build);

                next_factory.update_inventory(time_to_build);
                next_factory.add_robot(&material);

                if next_factory.max_geodes_count(remaining_time) > Some(best) {
                    best = best.max(
                        next_factory.geodes_count() + next_factory.geodes_rate() * remaining_time,
                    );

                    queue.push((next_factory, remaining_time))
                }
            }
        }

        best
    }
}

impl Display for Blueprint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self { id, robots, .. } = self;

        write!(
            f,
            "Blueprint {id}:\n{}",
            robots
                .iter()
                .map(|robot_cost| format!("\tEach {robot_cost}"))
                .join("\n")
        )
    }
}

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::blueprint::Blueprint;
use crate::material::{MatCount, Material, RobotCount};

#[derive(Debug, Clone)]
pub struct Factory<'a> {
    inventory: HashMap<Material, MatCount>,
    robots: HashMap<Material, RobotCount>,
    blueprint: &'a Blueprint,
}

impl<'a> Factory<'a> {
    pub fn new(blueprint: &'a Blueprint) -> Self {
        Self {
            inventory: Material::iter().map(|mat| (mat, 0.into())).collect(),
            robots: Material::iter()
                .map(|mat| {
                    let count = match mat {
                        Material::Ore => 1,
                        _ => 0,
                    }
                    .into();

                    (mat, count)
                })
                .collect(),
            blueprint,
        }
    }
}

impl Factory<'_> {
    pub fn geodes_count(&self) -> usize {
        self.inventory
            .get(&Material::Geode)
            .copied()
            .unwrap_or_default()
            .into()
    }

    pub fn geodes_rate(&self) -> usize {
        self.robots
            .get(&Material::Geode)
            .copied()
            .unwrap_or_default()
            .into()
    }

    pub fn max_geodes_count(&self, time: usize) -> Option<usize> {
        let geodes = self.inventory.get(&Material::Geode)?;
        let robots = self.robots.get(&Material::Obsidian)?;
        let obsidian = self.inventory.get(&Material::Obsidian)?;

        let cost = self
            .blueprint
            .get(&Material::Geode)?
            .cost
            .get(&Material::Obsidian)?
            .into_inner();

        let (_, _, max_production) = (1..time).rev().fold(
            (
                robots.into_inner(),
                obsidian.into_inner(),
                geodes.into_inner() + self.geodes_rate() * time,
            ),
            |(robots, obsidian, geodes), time| match obsidian.cmp(&cost) {
                Ordering::Less => (robots + 1, obsidian + robots, geodes),
                _ => (robots, obsidian - cost + robots, geodes + time),
            },
        );

        Some(max_production)
    }

    pub fn get_time_to_build(&self, robot: &Material) -> Option<usize> {
        let robot_cost = self.blueprint.get(robot)?;

        robot_cost
            .cost
            .iter()
            .filter_map(|(mat, &cost)| {
                let amount = self.inventory.get(mat)?;
                let robot_count = self.robots.get(mat)?;
                let remaining = cost.into_inner().saturating_sub(amount.into_inner());

                if robot_count.into_inner() == 0 {
                    return Some(usize::MAX);
                }

                let score = (remaining as f32) / (robot_count.into_inner() as f32);

                Some(score.ceil() as usize)
            })
            .max()
            .filter(|time| time < &usize::MAX)
    }

    pub fn should_construct(&self, robot: &Material) -> bool {
        let max_requirement = self.blueprint.max_requirement(robot).unwrap();
        let robot_count = self.robots.get(robot).unwrap();

        let is_ore_and_has_clays = match robot {
            //
            Material::Ore => self
                .robots
                .get(&Material::Clay)
                .filter(|amount| amount.into_inner() > 1)
                .is_some(),
            _ => false,
        };

        !is_ore_and_has_clays && robot_count.into_inner() < max_requirement.into_inner()
    }

    pub fn update_inventory(&mut self, time: usize) {
        self.inventory.iter_mut().for_each(|(mat, mat_count)| {
            let robot_count = self.robots.get(mat).unwrap();

            *mat_count += (robot_count.into_inner() * time).into();
        });
    }

    pub fn add_robot(&mut self, robot: &Material) {
        let robot_cost = self.blueprint.get(robot).unwrap();

        robot_cost.cost.iter().for_each(|(mat, cost)| {
            self.inventory.entry(*mat).and_modify(|mat_amount| {
                *mat_amount = mat_amount
                    .into_inner()
                    .saturating_sub(cost.into_inner())
                    .into();
            });
        });

        *self.robots.entry(*robot).or_insert_with(|| 0.into()) += RobotCount::from(1);
    }

    pub fn get_score(&self) -> usize {
        Material::iter().fold(0, |score, mat| {
            let count = self.robots.get(&mat).copied().unwrap_or_default();

            let multiplier = match mat {
                Material::Ore => 1,
                Material::Clay => 5,
                Material::Obsidian => 10,
                Material::Geode => 50,
            };

            score + count.into_inner() * multiplier
        })
    }
}

impl Display for Factory<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "You have:\n\t{}",
            Material::iter()
                .filter_map(|mat| {
                    let amount = self.inventory.get(&mat)?;
                    let robots = self.robots.get(&mat)?;

                    Some(format!("{mat}: {robots} robot + {amount}"))
                })
                .join("\n\t")
        )
    }
}

impl Eq for Factory<'_> {}

impl PartialEq<Self> for Factory<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.get_score() == other.get_score()
    }
}

impl PartialOrd<Self> for Factory<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Factory<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_score().cmp(&other.get_score())
    }
}

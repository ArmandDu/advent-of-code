use derive_more::{Add, AddAssign, Display, From, Into};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, Display, Hash, Eq, PartialEq, EnumIter, Ord, PartialOrd)]
pub enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, From, Into, Add, AddAssign, Copy, Clone, Display, Eq, PartialEq, Default)]
pub struct MatCount(usize);

impl MatCount {
    pub fn into_inner(self) -> usize {
        self.0
    }
}

#[derive(Debug, From, Into, Add, AddAssign, Copy, Clone, Display, Eq, PartialEq, Default)]
pub struct RobotCount(usize);

impl RobotCount {
    pub fn into_inner(self) -> usize {
        self.0
    }
}

#[derive(Debug)]
pub struct RobotCost {
    pub(crate) material: Material,
    pub(crate) cost: HashMap<Material, MatCount>,
}

impl RobotCost {
    pub fn new(material: Material, cost: impl Into<HashMap<Material, MatCount>>) -> Self {
        let cost = cost.into();

        Self { material, cost }
    }
}

impl Display for RobotCost {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} robot costs {}",
            self.material,
            Material::iter()
                .filter_map(|mat| Some(format!("{} {mat}", self.cost.get(&mat)?)))
                .join(" and ")
        )
    }
}

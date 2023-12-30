use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Matrix<T>(Vec<((usize, usize), T)>);

impl<T> Matrix<T> {
    pub fn inner(self) -> Vec<((usize, usize), T)> {
        self.0
    }

    pub fn width(&self) -> usize {
        self.0
            .iter()
            .map(|((x, _), _)| x + 1)
            .max()
            .unwrap_or_default()
    }
    pub fn height(&self) -> usize {
        self.0
            .iter()
            .map(|((_, y), _)| y + 1)
            .max()
            .unwrap_or_default()
    }

    pub fn get(&self, coord: &(usize, usize)) -> Option<&T> {
        self.0
            .iter()
            .find_map(|(k, v)| if coord == k { Some(v) } else { None })
    }

    pub fn get_mut(&mut self, coord: &(usize, usize)) -> Option<&mut T> {
        self.0
            .iter_mut()
            .find_map(|(k, v)| if coord == k { Some(v) } else { None })
    }

    ///iter on each row
    ///
    /// ```text
    /// -> a a a a a a a
    /// -> b b b b b b b
    /// -> c c c c c c c
    ///
    /// yields [a, a, ...] [b, b, ...] [c, c, ...]
    /// ```
    ///
    pub fn iter_row(&self) -> impl Iterator<Item = (usize, impl Iterator<Item = (&usize, &T)>)> {
        (0..self.height()).map(|k| {
            (
                k,
                self.0
                    .iter()
                    .filter(move |((_, y), _)| *y == k)
                    .map(|((x, _), c)| (x, c)),
            )
        })
    }

    ///iter on each column
    ///
    /// ```text
    /// | | | | | | |
    /// v v v v v v v
    /// a a a a a a a
    /// b b b b b b b
    /// c c c c c c c
    ///
    /// yields [a, b, c], [a, b, c] ....
    /// ```
    ///
    pub fn iter_col(&self) -> impl Iterator<Item = (usize, impl Iterator<Item = (&usize, &T)>)> {
        (0..self.width()).map(|k| {
            (
                k,
                self.0
                    .iter()
                    .filter(move |((x, _), _)| *x == k)
                    .map(|((_, y), c)| (y, c)),
            )
        })
    }

    /// iter on the matrix
    /// ```text
    /// yields [row * col]
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &((usize, usize), T)> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut ((usize, usize), T)> {
        self.0.iter_mut()
    }
}

impl<I: Iterator> FromIterator<I> for Matrix<I::Item> {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .enumerate()
                .flat_map(|(y, row)| row.enumerate().map(move |(x, c)| ((x, y), c)))
                .collect(),
        )
    }
}

impl From<&str> for Matrix<char> {
    fn from(value: &str) -> Self {
        value.lines().map(|row| row.chars()).collect()
    }
}

impl<T> From<HashMap<(usize, usize), T>> for Matrix<T> {
    fn from(value: HashMap<(usize, usize), T>) -> Self {
        Self(value.into_iter().collect())
    }
}

impl Display for Matrix<char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let width = self.width();
        let header = (0..width).join(" ");

        write!(
            f,
            "   {header}\n{}",
            self.iter_row()
                .map(|(y, row)| format!("{y:#2} {}", row.map(|(_, c)| c).join(" ")))
                .join("\n")
        )
    }
}

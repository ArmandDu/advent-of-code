use std::cmp::Ordering;
use std::str::FromStr;

use aoc::solution::SolutionError;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Point<T>(T, T);

impl<T: Copy> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> T {
        self.0
    }
    pub fn y(&self) -> T {
        self.1
    }
    pub fn xy(&self) -> (T, T) {
        (self.0, self.1)
    }
}

impl<T: Copy> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Self(x, y)
    }
}

impl<T: Copy + Ord> PartialOrd<Self> for Point<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Copy + Ord> Ord for Point<T> {
    ///
    /// # Examples
    /// ```
    /// # use std::cmp::Ordering;
    /// # use shared::Point;
    ///
    /// //   -------
    /// // 2 | | |c|
    /// //   -------
    /// // 1 | |a|b|
    /// //   -------
    /// // 0 | | | |
    /// //   -------
    /// //    0 1 2
    /// let a = Point::from((1,1));
    /// let b = Point::from((1,2));
    /// let c = Point::from((2,2));
    ///
    /// assert_eq!(a.cmp(&b), Ordering::Less);
    /// assert_eq!(a.cmp(&c), Ordering::Less);
    /// assert_eq!(b.cmp(&c), Ordering::Less);
    ///
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x().cmp(&other.x()) {
            Ordering::Equal => self.y().cmp(&other.y()),
            ord => ord,
        }
    }
}

impl<T: FromStr> FromStr for Point<T> {
    type Err = SolutionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use SolutionError::*;

        let (x, y) = value.split_once(',').ok_or(ParseError)?;
        let x = x.parse().or(Err(ParseError))?;
        let y = y.parse().or(Err(ParseError))?;

        Ok(Self(x, y))
    }
}

pub mod geometry {
    use std::iter::Map;
    use std::ops::RangeInclusive;

    use itertools::{Itertools, Product};

    use crate::Point;

    type LineIter<T> = Map<Product<RangeInclusive<T>, RangeInclusive<T>>, fn((T, T)) -> Point<T>>;

    impl Point<usize> {
        pub fn line_to(&self, other: &Self) -> LineIter<usize> {
            let min = self.min(other);
            let max = self.max(other);

            (min.x()..=max.x())
                .cartesian_product(min.y()..=max.y())
                .map(Point::from)
        }
    }

    impl Point<i32> {
        pub fn line_to(&self, other: &Self) -> LineIter<i32> {
            let min = self.min(other);
            let max = self.max(other);

            (min.x()..=max.x())
                .cartesian_product(min.y()..=max.y())
                .map(Point::from)
        }
    }
}

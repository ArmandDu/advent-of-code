use aoc::solution::SolutionError;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Point(usize, usize);

impl Point {
    
    pub fn new(x: usize,y:usize) -> Self{
        Self(x, y)
    }
    
    pub fn x(&self) -> usize {
        self.0
    }
    pub fn y(&self) -> usize {
        self.1
    }
    pub fn xy(&self) -> (usize, usize) {
        (self.0, self.1)
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self(x, y)
    }
}

impl PartialOrd<Self> for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
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

impl TryFrom<&str> for Point {
    type Error = SolutionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use SolutionError::*;

        let (x, y) = value.split_once(',').ok_or(ParseError)?;
        let x = x.parse().or(Err(ParseError))?;
        let y = y.parse().or(Err(ParseError))?;

        Ok(Self(x, y))
    }
}

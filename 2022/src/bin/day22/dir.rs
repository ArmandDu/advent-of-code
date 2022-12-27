use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Dir {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl From<Dir> for Axis {
    fn from(dir: Dir) -> Self {
        match dir {
            Dir::Up | Dir::Down => Self::Vertical,
            _ => Self::Horizontal,
        }
    }
}

impl Default for Dir {
    fn default() -> Self {
        Self::Right
    }
}

impl Dir {
    pub fn as_angle(&self) -> f64 {
        match self {
            Dir::Right => 0.,
            Dir::Down => 90.,
            Dir::Left => 180.,
            Dir::Up => 270.,
        }
    }

    pub fn forward(&self) -> (isize, isize) {
        let dx = self.as_angle().to_radians().cos() as isize;
        let dy = self.as_angle().to_radians().sin() as isize;

        (dx, dy)
    }

    pub fn right(&self) -> Self {
        *self + Self::from(90.)
    }
    pub fn left(&self) -> Self {
        *self - Self::from(90.)
    }

    pub fn flip(&self) -> Self {
        *self + Self::from(180.)
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Right, Self::Up, Self::Left, Self::Down].into_iter()
    }
}

impl From<f64> for Dir {
    fn from(angle: f64) -> Self {
        match (angle as i32) % 360 {
            0 => Self::Right,
            90 => Self::Down,
            180 => Self::Left,
            270 => Self::Up,
            _ => unreachable!(),
        }
    }
}

impl Add<Dir> for Dir {
    type Output = Self;

    fn add(self, rhs: Dir) -> Self::Output {
        ((360. + (self.as_angle() + rhs.as_angle())) % 360.).into()
    }
}

impl Sub<Dir> for Dir {
    type Output = Self;

    fn sub(self, rhs: Dir) -> Self::Output {
        ((360. + (self.as_angle() - rhs.as_angle())) % 360.).into()
    }
}

impl Add<(usize, usize)> for Dir {
    type Output = (usize, usize);

    fn add(self, (x, y): (usize, usize)) -> Self::Output {
        let (dx, dy) = self.forward();

        ((x as isize + dx) as usize, (y as isize + dy) as usize)
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::Right => '>',
                Dir::Down => 'v',
                Dir::Left => '<',
                Dir::Up => '^',
            }
        )
    }
}

impl From<Dir> for usize {
    fn from(d: Dir) -> Self {
        d.as_angle() as usize / 90
    }
}

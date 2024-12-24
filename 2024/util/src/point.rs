use std::cmp::Ordering;

use crate::direction::Direction;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Point(pub i32, pub i32);
impl Point {
    pub fn add(&self, point: &Point) -> Point {
        Point(self.0 + point.0, self.1 + point.1)
    }

    pub fn sub(&self, point: &Point) -> Point {
        Point(self.0 - point.0, self.1 - point.1)
    }
    
    pub fn diff(&self, point: &Point) -> Point {
        Point(point.0 - self.0, point.1 - self.1)
    }

    pub fn normalize(&self) -> Point {
        let Point(x, y) = self;
        let x = match x.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Equal => 0,
            Ordering::Less => -1,
        };
        let y = match y.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Equal => 0,
            Ordering::Less => -1,
        };
        Point(x, y)
    }

    pub fn abs(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }

    pub fn direction_of(&self, point: &Point) -> Option<Direction> {
        Direction::from_point(&self.diff(point))
    }
}

// TODO can I make this generic for any integer type?
impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self(x as i32, y as i32)
    }
}
impl From<Point> for (usize, usize) {
    fn from(value: Point) -> Self {
        (value.0 as usize, value.1 as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_of() {
        let p1 = Point(1, 0);
        let p2 = Point(2, 0);
        assert_eq!(p1.direction_of(&p2), Some(Direction::Down));
    }

    #[test]
    fn test_from_usize() {
        let point = Point::from((4usize, 9usize));
        assert_eq!(Point(4, 9), point);
    }

    #[test]
    fn test_into_usize() {
        let point = Point(4, 9);
        assert_eq!((4usize, 9usize), point.into());
    }
}
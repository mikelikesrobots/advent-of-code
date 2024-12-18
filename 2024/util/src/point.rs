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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_of() {
        let p1 = Point(1, 0);
        let p2 = Point(2, 0);
        assert_eq!(p1.direction_of(&p2), Some(Direction::Down));
    }
}
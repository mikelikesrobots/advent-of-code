use crate::point::Point;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn to_point(&self) -> Point {
        match self {
            Direction::Up => Point(-1, 0),
            Direction::UpRight => Point(-1, 1),
            Direction::Right => Point(0, 1),
            Direction::DownRight => Point(1, 1),
            Direction::Down => Point(1, 0),
            Direction::DownLeft => Point(1, -1),
            Direction::Left => Point(0, -1),
            Direction::UpLeft => Point(-1, -1),
        }
    }

    pub fn from_point(point: &Point) -> Option<Direction> {
        match point {
            Point(-1, 0) => Some(Direction::Up),
            Point(-1, 1) => Some(Direction::UpRight),
            Point(0, 1) => Some(Direction::Right),
            Point(1, 1) => Some(Direction::DownRight),
            Point(1, 0) => Some(Direction::Down),
            Point(1, -1) => Some(Direction::DownLeft),
            Point(0, -1) => Some(Direction::Left),
            Point(-1, -1) => Some(Direction::UpLeft),
            _ => None,
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::UpRight => Direction::DownLeft,
            Direction::Right => Direction::Left,
            Direction::DownRight => Direction::UpLeft,
            Direction::Down => Direction::Up,
            Direction::DownLeft => Direction::UpRight,
            Direction::Left => Direction::Right,
            Direction::UpLeft => Direction::DownRight,
        }
    }

    pub fn all_directions() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
        ]
    }

    pub fn horiz_and_vert() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }

    pub fn corners() -> Vec<Direction> {
        vec![
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownRight,
            Direction::DownLeft,
        ]
    }

    pub fn right90(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::UpRight => Direction::DownRight,
            Direction::Right => Direction::Down,
            Direction::DownRight => Direction::DownLeft,
            Direction::Down => Direction::Left,
            Direction::DownLeft => Direction::UpLeft,
            Direction::Left => Direction::Up,
            Direction::UpLeft => Direction::UpRight,
        }
    }

    pub fn left90(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::UpRight => Direction::UpLeft,
            Direction::Right => Direction::Up,
            Direction::DownRight => Direction::UpRight,
            Direction::Down => Direction::Right,
            Direction::DownLeft => Direction::DownRight,
            Direction::Left => Direction::Down,
            Direction::UpLeft => Direction::DownLeft,
        }
    }
}

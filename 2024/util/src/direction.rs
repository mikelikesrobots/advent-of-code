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
}

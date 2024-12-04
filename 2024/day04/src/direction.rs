use strum::{EnumIter, IntoEnumIterator};
use crate::point::Point;

#[derive(Debug, EnumIter)]
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
        Direction::iter().collect::<Vec<_>>()
    }

    pub fn corners() -> Vec<Direction> {
        vec![
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownRight,
            Direction::DownLeft,
        ]
    }
}

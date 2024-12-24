use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use itertools::Itertools;
use util::{direction::Direction, point::Point};

use crate::robot_parse_err::RobotParseErr;

#[derive(Debug, Clone)]
pub struct Robot {
    key_map: HashMap<(char, char), Vec<String>>,
}

impl FromStr for Robot {
    type Err = RobotParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: HashMap<Point, char> = s
            .lines()
            .enumerate()
            .flat_map(|(row_idx, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(col_idx, cell)| match cell {
                        ' ' => None,
                        cell => Some((Point::from((row_idx, col_idx)), cell)),
                    })
            })
            .collect();

        let mut key_map = HashMap::new();
        for (start_coord, start_key) in &coords {
            for (end_coord, end_key) in &coords {
                if start_key == end_key {
                    key_map.insert((*start_key, *end_key), vec!["A".to_string()]);
                    continue;
                }

                // Find the difference between the two points.
                let diff = start_coord.diff(end_coord);
                let mut direction_counts = HashMap::new();
                match diff.0.cmp(&0) {
                    Ordering::Greater => {
                        direction_counts.insert(Direction::Down, diff.0);
                    }
                    Ordering::Less => {
                        direction_counts.insert(Direction::Up, -diff.0);
                    }
                    _ => (),
                }

                match diff.1.cmp(&0) {
                    Ordering::Greater => {
                        direction_counts.insert(Direction::Right, diff.1);
                    }
                    Ordering::Less => {
                        direction_counts.insert(Direction::Left, -diff.1);
                    }
                    _ => (),
                }

                let paths = direction_counts.iter()
                    .map(|(dir, count)| {
                        let mut path = vec![];
                        (0..*count).for_each(|_| path.push(dir));
                        // If only one direction, return.
                        if direction_counts.len() >= 2 {
                            let (other_dir, other_count) = direction_counts.iter().filter(|(other_dir, _)| dir != *other_dir).nth(0).expect("Could not get second direction after checking length of directions");
                            (0..*other_count).for_each(|_| path.push(other_dir));
                        }
                        path
                    })
                    .filter(|path| {
                        path.iter().try_fold(*start_coord, |current, dir| {
                            let next = current.add(&dir.to_point());
                            coords.get(&next).map(|_| next)
                        }).is_some()
                    })
                    .map(|path| {
                        path.iter().map(|dir| {
                            match dir {
                                Direction::Up => '^',
                                Direction::Down => 'v',
                                Direction::Right => '>',
                                Direction::Left => '<',
                                _ => panic!("Unexpected direction while converting path to string"),
                            }
                        }).join("") + "A"
                    })
                    .collect();
                key_map.insert((*start_key, *end_key), paths);
            }
        }
        Ok(Robot {
            key_map,
        })
    }
}

impl Robot {
    pub fn segment(&self, from: char, to: char) -> Option<Vec<String>> {
        self.key_map.get(&(from, to)).cloned()
    }

    pub fn paths(&self, keys: &str) -> Option<Vec<String>> {
        let full_keys = "A".to_string() + keys;
        full_keys
            .chars()
            .tuple_windows()
            .try_fold(vec!["".to_string()], |acc, (from, to)| {
                self.segment(from, to).map(|paths| {
                    paths
                        .iter()
                        .flat_map(|path| {
                            acc.iter()
                                .map(|old_path: &String| old_path.to_owned() + path)
                                .collect::<Vec<_>>()
                        })
                        .collect()
                })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_keypad() {
        let robot = Robot::from_str("12\nA0").unwrap();
        println!("{:?}", robot);
        assert_eq!(robot.paths("120").unwrap(), vec!["^A>AvA".to_owned()]);
    }

    #[test]
    fn test_full_keypad() {
        let robot = Robot::from_str("789\n456\n123\n 0A").unwrap();
        println!("{:?}", robot);
        assert_eq!(robot.paths("312").unwrap(), vec!["^A<<A>A".to_owned()]);
    }
}

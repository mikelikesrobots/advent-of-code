use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use crate::{robot::Robot, robot_parse_err::RobotParseErr};

#[derive(Debug)]
pub struct KeypadSolver {
    robots: Vec<Robot>,
    cache: HashMap<(usize, char, char), usize>,
}

impl KeypadSolver {
    pub fn new_part_a() -> Result<Self, RobotParseErr> {
        let robots = [
            Robot::from_str("789\n456\n123\n 0A"),
            Robot::from_str(" ^A\n<v>"),
            Robot::from_str(" ^A\n<v>"),
        ]
        .into_iter()
        .collect::<Result<Vec<Robot>, RobotParseErr>>()?;
        Ok(Self { robots, cache: HashMap::new() })
    }

    pub fn new_part_b() -> Result<Self, RobotParseErr> {
        let mut robots = vec![Robot::from_str("789\n456\n123\n 0A")?];
        let simple_robot = Robot::from_str(" ^A\n<v>")?;
        (0..25).for_each(|_| robots.push(simple_robot.clone()));
        Ok(Self { robots, cache: HashMap::new() })
    }

    fn paths(&self, line: &str) -> Option<Vec<String>> {
        self.robots
            .iter()
            .try_fold(vec![line.to_string()], |acc, r| {
                let mut new_acc = vec![];
                for s in acc {
                    if let Some(mapped) = r.paths(&s) {
                        new_acc.push(mapped);
                    }
                }
                Some(new_acc.iter().flatten().map(|s| s.to_string()).collect())
            })
    }

    fn complexity(&self, line: &str) -> Option<usize> {
        let numeric: Option<usize> = line
            .chars()
            .filter(|x| x.is_ascii_digit())
            .collect::<String>()
            .parse()
            .ok();
        let path = self
            .paths(line)
            .and_then(|paths| paths.iter().map(|s| s.len()).min());
        match (numeric, &path) {
            (Some(numeric), Some(path)) => Some(numeric * path),
            _ => None,
        }
    }

    fn complexity_alt(&mut self, line: &str) -> Option<usize> {
        let numeric: Option<usize> = line
            .chars()
            .filter(|x| x.is_ascii_digit())
            .collect::<String>()
            .parse()
            .ok();
        let path = self.sum_keytaps_line(line);
        match (numeric, &path) {
            (Some(numeric), Some(path)) => Some(numeric * path),
            _ => None,
        }
    }

    fn min_length_path_for_char(
        &mut self,
        robot_level: usize,
        prev_char: char,
        target: char,
    ) -> Option<usize> {

        if let Some(x) = self.cache.get(&(robot_level, prev_char, target)){
            return Some(*x);
        }

        // If robot level is at end, return None - just to be sure
        if robot_level >= self.robots.len() {
            return None;
        }

        let min_length = self.robots[robot_level]
            .segment(prev_char, target)
            .and_then(|paths| {
                if robot_level == self.robots.len() - 1 {
                    return Some(paths[0].len());
                }
                paths.iter().map(|path| {
                    // For a path, figure out the length from that path upwards
                    let full_path = "A".to_string() + path;
                    full_path.chars().tuple_windows().try_fold(0, |acc, (from, to)| {
                        self.min_length_path_for_char(robot_level + 1, from, to)
                            .map(|x| acc + x)
                    })
                }).min().flatten()
            });
        if let Some(x) = min_length {
            self.cache.insert((robot_level, prev_char, target), x);
        }
        min_length
    }

    // Find the smallest complexity for a given line
    fn sum_keytaps_line(&mut self, line: &str) -> Option<usize> {
        let path = "A".to_string() + line;
        path.chars().tuple_windows().try_fold(0, |acc, (from, to)| {
            self.min_length_path_for_char(0, from, to).map(|x| acc + x)
        })
    }

    pub fn sum_keytaps(&self, puzzle: &str) -> usize {
        puzzle
            .lines()
            .filter_map(|line| self.complexity(line))
            .sum()
    }

    pub fn sum_keytaps_alt(&mut self, puzzle: &str) -> usize {
        puzzle
            .lines()
            .filter_map(|line| self.complexity_alt(line))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_txt() {
        let puzzle = include_str!("../puzzle/test.txt");
        let solver = KeypadSolver::new_part_a().unwrap();
        let result = solver.sum_keytaps(puzzle);
        assert_eq!(126384, result);
    }

    #[test]
    fn test_test_txt_alt() {
        let puzzle = include_str!("../puzzle/test.txt");
        let mut solver = KeypadSolver::new_part_a().unwrap();
        let result = solver.sum_keytaps_alt(puzzle);
        assert_eq!(126384, result);
    }
}

use std::collections::HashSet;
use std::{collections::HashMap, str::FromStr};

use util::direction::Direction;
use util::point::Point;

#[derive(Debug)]
struct PlantMap {
    regions: HashMap<char, Vec<Vec<Point>>>,
}

#[derive(Debug)]
enum PlantMapErr {}
impl FromStr for PlantMap {
    type Err = PlantMapErr;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = puzzle.lines().map(|l| l.chars().collect()).collect();

        let in_bounds = |point: &Point| {
            point.0 >= 0
                && point.1 >= 0
                && point.0 < (grid.len() as i32)
                && point.1 < (grid[0].len() as i32)
        };

        let mut regions: HashMap<char, Vec<Vec<Point>>> = HashMap::new();

        let mut explored = HashSet::new();
        let mut stack = vec![];
        let mut current_region = HashSet::new();
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, plant_type) in row.iter().enumerate() {
                let point = Point(row_idx as i32, col_idx as i32);
                if explored.contains(&point) {
                    continue;
                }

                stack.push(point);
                while let Some(point) = stack.pop() {
                    explored.insert(point);
                    current_region.insert(point);

                    // Check in all directions of point
                    let all_points = Direction::horiz_and_vert()
                        .into_iter()
                        .map(|d| point.add(&d.to_point()))
                        .filter(|p| !explored.contains(p))
                        .filter(&in_bounds)
                        .filter(|p| grid[p.0 as usize][p.1 as usize] == *plant_type);
                    stack.extend(all_points);
                }
                regions
                    .entry(*plant_type)
                    .or_default()
                    .push(current_region.clone().into_iter().collect::<Vec<Point>>());
                current_region.clear();
            }
        }

        Ok(PlantMap { regions })
    }
}

impl PlantMap {
    fn area(region: &[Point]) -> usize {
        region.len()
    }
    fn perimeter(region: &[Point]) -> usize {
        let mut perim = region.len() * 4;
        let dirs = {
            let mut dirs = HashSet::new();
            for dir in Direction::horiz_and_vert() {
                dirs.insert(dir.to_point());
            }
            dirs
        };
        for left_idx in 0..region.len() - 1 {
            for right_idx in left_idx + 1..region.len() {
                let left = &region[left_idx];
                let right = &region[right_idx];
                if dirs.contains(&right.diff(left)) {
                    perim -= 2;
                }
            }
        }
        perim
    }
    fn cost(region: &[Point]) -> usize {
        PlantMap::area(region) * PlantMap::perimeter(region)
    }

    fn part_a(&self) -> usize {
        self.regions
            .values()
            .map(|regions| {
                regions
                    .iter()
                    .map(|region| PlantMap::cost(region))
                    .sum::<usize>()
            })
            .sum()
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let map = PlantMap::from_str(puzzle).unwrap();
    println!("Part A: {}", map.part_a());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_1_txt() {
        let puzzle = include_str!("../puzzle/test_1.txt");
        let map = PlantMap::from_str(puzzle).unwrap();
        let result = map.part_a();
        assert_eq!(140, result);
    }

    #[test]
    fn test_test_2_txt() {
        let puzzle = include_str!("../puzzle/test_2.txt");
        let map = PlantMap::from_str(puzzle).unwrap();
        let result = map.part_a();
        assert_eq!(772, result);
    }

    #[test]
    fn test_test_3_txt() {
        let puzzle = include_str!("../puzzle/test_3.txt");
        let map = PlantMap::from_str(puzzle).unwrap();
        let result = map.part_a();
        assert_eq!(1930, result);
    }
}

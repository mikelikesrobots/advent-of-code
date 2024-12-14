use std::cmp::Ordering;
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
    fn perimeter_with_discount(region: &[Point]) -> usize {
        let mut fences = 0;

        // Scan from top to bottom
        let mut current_row: Vec<i32> = vec![];
        let mut prev_row: Vec<i32> = vec![];
        let mut points = region.to_vec();
        points.sort();

        let count_distinct = |v: &Vec<i32>| {
            let mut distinct_vals = 0;
            let mut prev_val = None;
            for val in v {
                if let Some(prev) = prev_val {
                    if val.abs_diff(prev) != 1 {
                        distinct_vals += 1;
                    }
                    prev_val = Some(*val);
                } else {
                    distinct_vals += 1;
                    prev_val = Some(*val);
                }
            }
            distinct_vals
        };

        let extra_fences = |current_row: &Vec<i32>, prev_row: &Vec<i32>| {
            // Diff the rows
            let new_current_vals: Vec<i32> = current_row.iter().filter(|p| !prev_row.contains(p)).copied().collect();
            let new_prev_vals: Vec<i32> = prev_row.iter().filter(|p| !current_row.contains(p)).copied().collect();

            // Return distinct count from both vector
            count_distinct(&new_current_vals) + count_distinct(&new_prev_vals)
        };

        let mut prev_row_idx = -1;
        while let Some(point) = points.pop() {
            let current_row_idx = point.0;
            if current_row_idx != prev_row_idx {
                fences += extra_fences(&current_row, &prev_row);

                prev_row = current_row.to_vec();
                current_row.clear();
                current_row.push(point.1);
                prev_row_idx = current_row_idx;
            } else {
                current_row.push(point.1);
            }
        }

        fences += extra_fences(&prev_row, &current_row);
        fences += extra_fences(&current_row, &vec![]);

        // Scan from left to right
        let mut points = region.to_vec();
        points.sort_by(|a, b| {
            match (a.0.cmp(&b.0), a.1.cmp(&b.1)) {
                (_, Ordering::Greater) => Ordering::Greater,
                (_, Ordering::Less) => Ordering::Less,
                (Ordering::Greater, _) => Ordering::Greater,
                (Ordering::Less, _) => Ordering::Less,
                (Ordering::Equal, Ordering::Equal) => Ordering::Equal,
            }
        });

        let mut prev_col_idx = -1;
        let mut prev_col = vec![];
        let mut current_col = vec![];
        while let Some(point) = points.pop() {
            let current_col_idx = point.1;
            if current_col_idx != prev_col_idx {
                fences += extra_fences(&current_col, &prev_col);

                prev_col = current_col.to_vec();
                current_col.clear();
                current_col.push(point.0);
                prev_col_idx = current_col_idx;
            } else {
                current_col.push(point.0);
            }
        }

        fences += extra_fences(&prev_col, &current_col);
        fences += extra_fences(&current_col, &vec![]);

        fences
    }
    fn cost(region: &[Point]) -> usize {
        PlantMap::area(region) * PlantMap::perimeter(region)
    }
    fn cost_with_discount(region: &[Point]) -> usize {
        PlantMap::area(region) * PlantMap::perimeter_with_discount(region)
    }

    fn calc_cost<F>(&self, cost_fn: F) -> usize
    where F: Fn(&Vec<Point>) -> usize {
        self.regions
            .values()
            .map(|regions| {
                regions
                    .iter()
                    .map(&cost_fn)
                    .sum::<usize>()
            })
            .sum()
    }

    fn part_a(&self) -> usize {
        self.calc_cost(|r| PlantMap::cost(r))
    }

    fn part_b(&self) -> usize {
        self.calc_cost(|r| PlantMap::cost_with_discount(r))
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let map = PlantMap::from_str(puzzle).unwrap();
    println!("Part A: {}", map.part_a());
    println!("Part B: {}", map.part_b());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_1_txt_part_a() {
        let puzzle = include_str!("../puzzle/test_1.txt");
        let map = PlantMap::from_str(puzzle).unwrap();
        let result = map.part_a();
        assert_eq!(140, result);
    }

    #[test]
    fn test_test_2_txt_part_a() {
        let puzzle = include_str!("../puzzle/test_2.txt");
        let map = PlantMap::from_str(puzzle).unwrap();
        let result = map.part_a();
        assert_eq!(772, result);
    }

    #[test]
    fn test_test_3_txt_part_a() {
        let puzzle = include_str!("../puzzle/test_3.txt");
        let map = PlantMap::from_str(puzzle).unwrap();
        let result = map.part_a();
        assert_eq!(1930, result);
    }

    #[test]
    fn test_test_1_txt_part_b() {
        let puzzle = include_str!("../puzzle/test_1.txt");
        let map = PlantMap::from_str(puzzle).unwrap();
        let result = map.part_b();
        assert_eq!(80, result);
    }

    #[test]
    fn test_test_2_txt_part_b() {
        let puzzle = include_str!("../puzzle/test_2.txt");
        let map = PlantMap::from_str(puzzle).unwrap();
        let result = map.part_b();
        assert_eq!(436, result);
    }

    #[test]
    fn test_test_3_txt_part_b() {
        let puzzle = include_str!("../puzzle/test_3.txt");
        let map = PlantMap::from_str(puzzle).unwrap();
        let result = map.part_b();
        assert_eq!(1206, result);
    }
}

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
        let mut map: HashMap<char, Vec<Point>> = HashMap::new();
        for (row_idx, row) in puzzle.lines().enumerate() {
            for (col_idx, plant_type) in row.chars().enumerate() {
                map.entry(plant_type)
                    .or_default()
                    .push(Point(row_idx as i32, col_idx as i32));
            }
        }

        let regions = {
            let mut regions = HashMap::new();
            map.iter().for_each(|(&plant_type, locations)| {
                _ = regions.insert(plant_type, PlantMap::group_contiguous(locations))
            });
            regions
        };
        Ok(PlantMap { regions })
    }
}

impl PlantMap {
    fn group_contiguous(locations: &[Point]) -> Vec<Vec<Point>> {
        let mut output = vec![];

        let touches = |point: &Point, region: &Vec<Point>| {
            for d in Direction::horiz_and_vert() {
                if region.contains(&point.add(&d.to_point())) {
                    return true;
                }
            }
            false
        };
        for location in locations {
            // Check if it matches an existing contiguous region
            if let Some(region) = output.iter_mut().find(|region| touches(location, region)) {
                region.push(*location);
                let mut combine_left_idx = Some(0);
                while combine_left_idx.is_some() {
                    combine_left_idx = None;
                    let mut combine_right_idx = None;
                    'outer: for left_idx in 0..output.len() - 1 {
                        for right_idx in left_idx + 1..output.len() {
                            let left_region = &output[left_idx];
                            let right_region = &output[right_idx];
                            for p in right_region.iter() {
                                if touches(p, left_region) {
                                    combine_left_idx = Some(left_idx);
                                    combine_right_idx = Some(right_idx);
                                    break 'outer;
                                }
                            }
                        }
                    }

                    if combine_left_idx.is_some() {
                        let right_region = output.remove(combine_right_idx.unwrap());
                        output[combine_left_idx.unwrap()].extend(right_region);
                    }
                }
            } else {
                output.push(vec![*location]);
            }
        }

        output
    }

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

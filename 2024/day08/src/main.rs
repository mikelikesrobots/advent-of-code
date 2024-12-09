use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};
use util::point::Point;

#[derive(Debug)]
struct AntennaMap {
    antennae: HashMap<char, Vec<Point>>,
    width: i32,
    height: i32,
}

#[derive(Debug)]
enum AntennaMapErr {}

impl FromStr for AntennaMap {
    type Err = AntennaMapErr;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let height = puzzle.lines().count() as i32;
        let width = puzzle.lines().nth(0).unwrap().len() as i32;

        let mut antennae = HashMap::new();
        for (row_idx, row) in puzzle.lines().enumerate() {
            for (col_idx, cell) in row.chars().enumerate() {
                match cell {
                    '.' => continue,
                    _ => {
                        antennae
                            .entry(cell)
                            .or_insert(vec![])
                            .push(Point(row_idx as i32, col_idx as i32));
                    }
                }
            }
        }

        Ok(AntennaMap {
            antennae,
            width,
            height,
        })
    }
}

impl AntennaMap {
    fn in_bounds(&self, point: &Point) -> bool {
        point.0 >= 0 && point.1 >= 0 && point.0 < self.height && point.1 < self.width
    }

    fn part_a(&self) -> usize {
        let mut antinodes = HashSet::new();
        for antennae in self.antennae.values() {
            for (left_idx, left) in antennae.iter().enumerate() {
                for right in antennae.iter().skip(left_idx + 1) {

                    let diff = left.diff(right);
                    let node_0 = left.sub(&diff);
                    if self.in_bounds(&node_0) {
                        antinodes.insert(node_0);
                    }
                    let node_1 = right.add(&diff);
                    if self.in_bounds(&node_1) {
                        antinodes.insert(node_1);
                    }
                }
            }
        }
        antinodes.len()
    }

    fn part_b(&self) -> usize {
        let mut antinodes = HashSet::new();
        for antennae in self.antennae.values() {
            for (left_idx, left) in antennae.iter().enumerate() {
                for right in antennae.iter().skip(left_idx + 1) {
                    let diff = left.diff(right);

                    let mut node = *left;
                    while self.in_bounds(&node) {
                        antinodes.insert(node);
                        node = node.sub(&diff);
                    }

                    node = *right;
                    while self.in_bounds(&node) {
                        antinodes.insert(node);
                        node = node.add(&diff);
                    }
                }
            }
        }
        antinodes.len()
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let map = AntennaMap::from_str(puzzle).unwrap();
    println!("Part A: {}", map.part_a());
    println!("Part B: {}", map.part_b());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_small_txt_part_a() {
        let puzzle = include_str!("../puzzle/test_small.txt");
        let map = AntennaMap::from_str(puzzle).unwrap();
        assert_eq!(2, map.part_a());
    }

    #[test]
    fn test_test_small_txt_part_b() {
        let puzzle = include_str!("../puzzle/test_small.txt");
        let map = AntennaMap::from_str(puzzle).unwrap();
        assert_eq!(5, map.part_b());
    }

    #[test]
    fn test_test_txt_part_a() {
        let puzzle = include_str!("../puzzle/test.txt");
        let map = AntennaMap::from_str(puzzle).unwrap();
        assert_eq!(14, map.part_a());
    }

    #[test]
    fn test_test_txt_part_b() {
        let puzzle = include_str!("../puzzle/test.txt");
        let map = AntennaMap::from_str(puzzle).unwrap();
        assert_eq!(34, map.part_b());
    }
}

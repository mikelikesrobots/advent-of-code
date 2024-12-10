use std::{collections::HashSet, num::ParseIntError, str::FromStr};
use util::{direction::Direction, point::Point};

#[derive(Debug)]
struct TopographicMap {
    grid: Vec<Vec<u32>>,
}

impl FromStr for TopographicMap {
    type Err = ParseIntError;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let grid = puzzle
            .lines()
            .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect();
        Ok(TopographicMap { grid })
    }
}

impl TopographicMap {
    fn in_bounds(&self, point: &Point) -> bool {
        point.0 >= 0
            && point.1 >= 0
            && (point.0 as usize) < self.grid.len()
            && (point.1 as usize) < self.grid[0].len()
    }

    fn count_trailheads(&self, point: &Point) -> usize {
        let mut trail_ends = HashSet::new();
        Direction::horiz_and_vert()
            .iter()
            .map(|dir| self.count_trailheads_inner(0, &point.add(&dir.to_point())))
            .for_each(|set| trail_ends.extend(set));
        trail_ends.len()
    }

    fn count_trailheads_inner(&self, prev_val: u32, point: &Point) -> HashSet<Point> {
        let mut trail_ends = HashSet::new();
        if !self.in_bounds(point) {
            return trail_ends;
        }
        let cell = self.grid[point.0 as usize][point.1 as usize];
        if cell != prev_val + 1 {
            return trail_ends;
        }
        if cell == 9 {
            trail_ends.insert(*point);
            return trail_ends;
        }

        Direction::horiz_and_vert()
            .iter()
            .map(|dir| self.count_trailheads_inner(cell, &point.add(&dir.to_point())))
            .for_each(|set| trail_ends.extend(set));

        trail_ends
    }


    fn trailhead_ratings(&self, point: &Point) -> usize {
        Direction::horiz_and_vert()
            .iter()
            .map(|dir| self.trailhead_ratings_inner(0, &point.add(&dir.to_point())))
            .sum()
    }

    fn trailhead_ratings_inner(&self, prev_val: u32, point: &Point) -> usize {
        if !self.in_bounds(point) {
            return 0;
        }
        let cell = self.grid[point.0 as usize][point.1 as usize];
        if cell != prev_val + 1 {
            return 0;
        }
        if cell == 9 {
            return 1;
        }

        Direction::horiz_and_vert()
            .iter()
            .map(|dir| self.trailhead_ratings_inner(cell, &point.add(&dir.to_point())))
            .sum()
    }


    fn find_zeroes(&self) -> Vec<Point> {
        let mut zeroes = vec![];
        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    zeroes.push(Point(row_idx as i32, col_idx as i32));
                }
            }
        }
        zeroes
    }

    fn part_a(&self) -> usize {
        self.find_zeroes()
            .iter()
            .map(|p| self.count_trailheads(p))
            .sum()
    }

    fn part_b(&self) -> usize {
        self.find_zeroes()
        .iter()
        .map(|p| self.trailhead_ratings(p))
        .sum()
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let map = TopographicMap::from_str(puzzle).expect("Failed to read map");
    println!("Part A: {}", map.part_a());
    println!("Part B: {}", map.part_b());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_small_txt() {
        let puzzle = include_str!("../puzzle/test_small.txt");
        let map = TopographicMap::from_str(puzzle).unwrap();
        let result = map.part_a();
        assert_eq!(1, result);
    }

    #[test]
    fn test_test_txt_part_a() {
        let puzzle = include_str!("../puzzle/test.txt");
        let map = TopographicMap::from_str(puzzle).unwrap();
        let result = map.part_a();
        assert_eq!(36, result);
    }

    #[test]
    fn test_test_txt_part_b() {
        let puzzle = include_str!("../puzzle/test.txt");
        let map = TopographicMap::from_str(puzzle).unwrap();
        let result = map.part_b();
        assert_eq!(81, result);
    }
}

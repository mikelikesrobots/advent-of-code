use std::{collections::HashMap, str::FromStr};

use pathfinding::{grid::Grid, prelude::dijkstra};

#[derive(Debug)]
struct RaceMaze {
    grid: Grid,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Debug)]
enum RaceMazeErr {
    NoStartFound,
    NoEndFound,
    UnrecognisedCharacter,
    GridBuildFailure,
    NoPathFound,
}

impl FromStr for RaceMaze {
    type Err = RaceMazeErr;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;
        let mut grid_coords = vec![];
        for (row_idx, line) in puzzle.lines().enumerate() {
            if !line.contains(".") {
                // Skip pure walls
                continue;
            }
            for (col_idx, cell) in line.chars().enumerate().skip(1) {
                let point = (col_idx - 1, row_idx - 1);
                match cell {
                    '.' => grid_coords.push(point),
                    'S' => {
                        grid_coords.push(point);
                        start = Some(point);
                    },
                    'E' => {
                        grid_coords.push(point);
                        end = Some(point);
                    },
                    '#' => (),
                    _ => return Err(RaceMazeErr::UnrecognisedCharacter),
                }
            }
        }

        if start.is_none() {
            return Err(RaceMazeErr::NoStartFound);
        }
        let start = start.unwrap();

        if end.is_none() {
            return Err(RaceMazeErr::NoEndFound);
        }
        let end = end.unwrap();

        let grid = Grid::from_coordinates(&grid_coords).ok_or(RaceMazeErr::GridBuildFailure)?;

        Ok(RaceMaze { grid, start, end })
    }
}

impl RaceMaze {
    // Think about how to solve this using a much faster method - sitting and waiting isn't going to cut it
    fn part_a(&self, threshold: usize) -> Result<usize, RaceMazeErr> {
        // Get base case
        let base_time: usize = dijkstra(&self.start, |p| self.grid.neighbours(*p).iter().map(|p| (*p, 1)).collect::<Vec<_>>(), |p| *p == self.end).ok_or(RaceMazeErr::NoPathFound)?.1;

        // Iterate through all non-points in grid, remove each, and find the shortest path
        let mut times_saved = HashMap::new();
        let candidates: Vec<_> = {
            let mut grid = self.grid.clone();
            grid.invert();
            grid.iter().collect()
        };

        for cheat_cand in candidates {
            let mut grid = self.grid.clone();
            grid.add_vertex(cheat_cand);
            if let Some((_, time)) = dijkstra(&self.start, |p| grid.neighbours(*p).iter().map(|p| (*p, 1)).collect::<Vec<_>>(), |p| *p == self.end) {
                let time_saved = base_time - time;
                *times_saved.entry(time_saved).or_insert(0) += 1;
            }
        }

        let cheats_saving_threshold = times_saved.iter().filter_map(|(time, count)| {
            if *time >= threshold {
                return Some(count);
            }
            None
        }).sum();
        Ok(cheats_saving_threshold)
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let maze = RaceMaze::from_str(puzzle).expect("Failed to parse puzzle");
    println!("Part A: {:?}", maze.part_a(100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_txt() {
        let puzzle = include_str!("../puzzle/test.txt");
        let maze = RaceMaze::from_str(puzzle).unwrap();
        assert_eq!(maze.part_a(36).unwrap(), 4);
    }
}
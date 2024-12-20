use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use pathfinding::{
    grid::Grid,
    prelude::{dijkstra, dijkstra_reach},
};

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
                    }
                    'E' => {
                        grid_coords.push(point);
                        end = Some(point);
                    }
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
    fn part_a(&self, threshold: usize) -> Result<usize, RaceMazeErr> {
        // Get base case
        let (base_path, base_time) = dijkstra(
            &self.start,
            |p| {
                self.grid
                    .neighbours(*p)
                    .iter()
                    .map(|p| (*p, 1))
                    .collect::<Vec<_>>()
            },
            |p| *p == self.end,
        )
        .ok_or(RaceMazeErr::NoPathFound)?;

        // Iterate through all non-points in grid, remove each, and find the shortest path
        let mut times_saved = HashMap::new();
        let mut inverted = self.grid.clone();
        inverted.invert();
        for point in &base_path {
            inverted.add_vertex(*point);
        }

        let mut grid = self.grid.clone();
        for (idx, point) in base_path.iter().enumerate() {
            for cheat_cand in inverted.neighbours(*point) {
                if base_path.contains(&cheat_cand) || *point == self.end {
                    continue;
                }
                grid.add_vertex(cheat_cand);
                if let Some((_, time)) = dijkstra(
                    point,
                    |p| {
                        grid.neighbours(*p)
                            .iter()
                            .map(|p| (*p, 1))
                            .collect::<Vec<_>>()
                    },
                    |p| *p == self.end,
                ) {
                    let time_saved = base_time - time - idx;
                    *times_saved.entry(time_saved).or_insert(0) += 1;
                }
                grid.remove_vertex(cheat_cand);
            }
        }

        let cheats_saving_threshold = times_saved
            .iter()
            .filter_map(|(time, count)| {
                if *time >= threshold {
                    return Some(count);
                }
                None
            })
            .sum();
        Ok(cheats_saving_threshold)
    }

    fn part_a_alt(&self, threshold: usize) -> Result<usize, RaceMazeErr> {
        // Get everywhere reachable by end
        let reachable: Vec<_> = dijkstra_reach(&self.end, |p, _| {
            self.grid
                .neighbours(*p)
                .iter()
                .map(|p| (*p, 1))
                .collect::<Vec<_>>()
        })
        .collect();

        let (base_path, base_time) = dijkstra(
            &self.start,
            |p| {
                self.grid
                    .neighbours(*p)
                    .iter()
                    .map(|p| (*p, 1))
                    .collect::<Vec<_>>()
            },
            |p| *p == self.end,
        )
        .ok_or(RaceMazeErr::NoPathFound)?;
        let base_time: usize = base_time;

        // Build grid where walls are the points
        let mut inverted = self.grid.clone();
        inverted.invert();
        for point in &base_path {
            inverted.add_vertex(*point);
        }

        let mut filled_grid = self.grid.clone();
        filled_grid.fill();

        let mut times_saved = HashMap::new();

        for (idx, cell) in base_path.iter().enumerate() {
            // Find neighbours that are not in the base path to test removing
            let cheat_cands: HashSet<_> = inverted
                .neighbours(*cell)
                .iter()
                .filter(|v| !base_path.contains(v))
                .copied()
                .collect();
            let end_points: HashSet<_> = cheat_cands
                .iter()
                .flat_map(|v| filled_grid.neighbours(*v))
                .filter(|v| v != cell && self.grid.has_vertex(*v))
                .collect();
            for end_point in end_points {
                if let Some(x) = reachable.iter().find(|node| node.node == end_point) {
                    let time_taken = idx + x.total_cost + 2;
                    let time_saved = base_time.saturating_sub(time_taken);
                    *times_saved.entry(time_saved).or_default() += 1;
                }
            }
        }

        let cheats_saving_threshold = times_saved
            .iter()
            .filter_map(|(time, count)| {
                if *time >= threshold {
                    return Some(count);
                }
                None
            })
            .sum();
        Ok(cheats_saving_threshold)
    }

    fn part_b(&self, dist_threshold: usize, time_threshold: usize) -> Result<usize, RaceMazeErr> {
        // Get everywhere reachable by end
        let reachable: Vec<_> = dijkstra_reach(&self.end, |p, _| {
            self.grid
                .neighbours(*p)
                .iter()
                .map(|p| (*p, 1))
                .collect::<Vec<_>>()
        })
        .collect();

        let (base_path, base_time) = dijkstra(
            &self.start,
            |p| {
                self.grid
                    .neighbours(*p)
                    .iter()
                    .map(|p| (*p, 1))
                    .collect::<Vec<_>>()
            },
            |p| *p == self.end,
        )
        .ok_or(RaceMazeErr::NoPathFound)?;
        let base_time: usize = base_time;

        let mut times_saved = HashMap::new();

        for (idx, cell) in base_path.iter().enumerate() {
            let cheat_cands: HashSet<_> = self.grid
                .iter()
                .filter(|v| {
                    self.grid.distance(*v, *cell) <= dist_threshold
                })
                .collect();

            for cheat_cand in cheat_cands {
                if let Some(x) = reachable.iter().find(|node| node.node == cheat_cand) {
                    let time_taken = idx + x.total_cost + self.grid.distance(*cell, cheat_cand);
                    let time_saved = base_time.saturating_sub(time_taken);
                    *times_saved.entry(time_saved).or_default() += 1;
                }
            }
        }

        let cheats_saving_threshold = times_saved
            .iter()
            .filter_map(|(time, count)| {
                if *time >= time_threshold {
                    return Some(count);
                }
                None
            })
            .sum();
        Ok(cheats_saving_threshold)
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let maze = RaceMaze::from_str(puzzle).expect("Failed to parse puzzle");
    // println!("Part A: {:?}", maze.part_a(100));
    // println!("Part A alt: {:?}", maze.part_a_alt(100));
    println!("Part A alt2: {:?}", maze.part_b(2, 100));
    println!("Part B: {:?}", maze.part_b(20, 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_txt_part_a() {
        let puzzle = include_str!("../puzzle/test.txt");
        let maze = RaceMaze::from_str(puzzle).unwrap();
        assert_eq!(maze.part_a_alt(36).unwrap(), 4);
    }

    #[test]
    fn test_test_txt_part_b() {
        let puzzle = include_str!("../puzzle/test.txt");
        let maze = RaceMaze::from_str(puzzle).unwrap();
        assert_eq!(maze.part_b(20, 50).unwrap(), 285);
    }
}

use std::{collections::HashSet, str::FromStr};

use util::{direction::Direction, point::Point};

use crate::{maze_cell::MazeCell, reindeer_maze_err::ReindeerMazeErr};

#[derive(Debug)]
pub struct ReindeerJunctionMaze {
    grid: Vec<Vec<MazeCell>>,
    junctions: HashSet<Point>,
    reindeer_pos: Point,
}

impl FromStr for ReindeerJunctionMaze {
    type Err = ReindeerMazeErr;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let mut end_found = false;
        let mut reindeer_pos = None;
        let mut grid = vec![];
        for (row_idx, row) in puzzle.lines().enumerate() {
            let mut maze_row = vec![];
            for (col_idx, cell) in row.chars().enumerate() {
                let point = Point(row_idx as i32, col_idx as i32);
                match cell {
                    '#' => {
                        maze_row.push(MazeCell::Wall);
                        continue;
                    }
                    'E' => {
                        end_found = true;
                        maze_row.push(MazeCell::End);
                    }
                    '.' => maze_row.push(MazeCell::Empty),
                    'S' => {
                        reindeer_pos = Some(point);
                        maze_row.push(MazeCell::Start);
                    }
                    _ => return Err(ReindeerMazeErr::UnrecognisedMazeChar),
                }
            }
            grid.push(maze_row);
        }

        // Build list of junctions
        let mut junctions = HashSet::new();
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if *cell == MazeCell::Wall {
                    continue;
                }
                let point = Point(row_idx as i32, col_idx as i32);
                let neighbours: Vec<Point> = Direction::horiz_and_vert()
                    .iter()
                    .filter_map(|d| {
                        let neighbour_point = point.add(&d.to_point());
                        if let Some(Some(neighbour)) = grid
                            .get(neighbour_point.0 as usize)
                            .map(|row| row.get(neighbour_point.1 as usize))
                        {
                            if *neighbour != MazeCell::Wall {
                                return Some(neighbour_point);
                            }
                        }
                        None
                    })
                    .collect();
                if neighbours.len() != 2 {
                    junctions.insert(point);
                    continue;
                }
                if let (Some(first_point), Some(last_point)) =
                    (neighbours.first(), neighbours.last())
                {
                    if last_point.direction_of(first_point).is_some() {
                        junctions.insert(point);
                    }
                }
            }
        }

        if reindeer_pos.is_none() {
            return Err(ReindeerMazeErr::NoStartFound);
        }
        let reindeer_pos = reindeer_pos.unwrap();
        if !end_found {
            return Err(ReindeerMazeErr::NoEndFound);
        }

        Ok(Self {
            grid,
            junctions,
            reindeer_pos,
        })
    }
}

impl ReindeerJunctionMaze {
    fn path_cost(visited: &[Point]) -> Option<usize> {
        if visited.is_empty() {
            return None;
        }

        let mut current_direction = Direction::Right;
        let mut cost = 0;
        let mut prev_point = visited.first().expect("Empty visited path");
        for point in visited.iter().skip(1) {
            let diff = prev_point.diff(point);
            let dir_opt = Direction::from_point(&diff.normalize());
            if let Some(dir) = dir_opt {
                if dir == current_direction {
                    cost += diff.abs();
                } else if dir == current_direction.right90() {
                    cost += 1000 + diff.abs();
                    current_direction = current_direction.right90();
                } else if dir == current_direction.left90() {
                    cost += 1000 + diff.abs();
                    current_direction = current_direction.left90();
                }
            }
            prev_point = point;
        }

        Some(cost as usize)
    }

    fn in_bounds(&self, point: &Point) -> bool {
        point.0 >= 0
            && point.1 >= 1
            && point.0 < (self.grid.len() as i32)
            && point.1 < (self.grid[0].len() as i32)
    }

    fn possible_points(&self, source: &Point, path: &[Point]) -> Vec<Point> {
        Direction::horiz_and_vert()
            .into_iter()
            .filter_map(|d| {

                let next_point = source.add(&d.to_point());
                if path.contains(&next_point) {
                    return None;
                }

                let contents = &self.grid[next_point.0 as usize][next_point.1 as usize];
                match contents {
                    MazeCell::Wall | MazeCell::Start => return None,
                    MazeCell::End => return Some(next_point),
                    _ => (),
                }

                // We are empty space, so keep going until we reach a junction
                let mut current = next_point;
                while !self.junctions.contains(&current) {
                    current = current.add(&d.to_point());
                    if path.contains(&current) {
                        return None;
                    }
                }
                Some(current)
            })
            .collect()
    }

    fn find_cheapest_iterative(&self) -> Option<usize> {
        let mut min_cost = None;
        let mut candidates: Vec<Vec<Point>> = vec![vec![self.reindeer_pos]];

        while let Some(candidate) = candidates.pop() {
            if candidate.last().is_none() {
                continue;
            }
            let last = candidate.last().expect("Found empty last after None check");

            // Check if last point is End
            if self.grid[last.0 as usize][last.1 as usize] == MazeCell::End {
                if let Some(cost) = ReindeerJunctionMaze::path_cost(&candidate) {
                    min_cost = min_cost.or(Some(usize::MAX)).map(|x| x.min(cost));
                }
                continue;
            }

            // If not end, add all paths from here as candidates
            self.possible_points(last, &candidate)
                .into_iter()
                .for_each(|p| {
                    let mut path = candidate.to_vec();
                    path.push(p);
                    candidates.push(path);
                });
        }

        min_cost
    }

    pub fn part_a(&self) -> Option<usize> {
        self.find_cheapest_iterative()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_very_small_txt() {
        let puzzle = include_str!("../puzzle/test_very_small.txt");
        let maze = ReindeerJunctionMaze::from_str(puzzle).unwrap();
        assert_eq!(maze.part_a(), Some(2003));
    }

    #[test]
    fn test_test_small_txt() {
        let puzzle = include_str!("../puzzle/test_small.txt");
        let maze = ReindeerJunctionMaze::from_str(puzzle).unwrap();
        assert_eq!(maze.part_a(), Some(7036));
    }

    #[test]
    fn test_test_large_txt() {
        let puzzle = include_str!("../puzzle/test_large.txt");
        let maze = ReindeerJunctionMaze::from_str(puzzle).unwrap();
        assert_eq!(maze.part_a(), Some(11048));
    }
}

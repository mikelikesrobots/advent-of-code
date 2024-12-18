use std::str::FromStr;

use util::{direction::Direction, point::Point};

use crate::{maze_cell::MazeCell, reindeer_maze_err::ReindeerMazeErr};

#[derive(Debug)]
pub struct ReindeerMaze {
    grid: Vec<Vec<MazeCell>>,
    reindeer_pos: Point,
}

impl FromStr for ReindeerMaze {
    type Err = ReindeerMazeErr;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let mut end_found = false;
        let mut reindeer_pos = None;
        let mut grid = vec![];
        for (row_idx, row) in puzzle.lines().enumerate() {
            let mut maze_row = vec![];
            for (col_idx, cell) in row.chars().enumerate() {
                match cell {
                    '#' => maze_row.push(MazeCell::Wall),
                    'E' => {
                        end_found = true;
                        maze_row.push(MazeCell::End);
                    }
                    '.' => maze_row.push(MazeCell::Empty),
                    'S' => {
                        reindeer_pos = Some(Point(row_idx as i32, col_idx as i32));
                        maze_row.push(MazeCell::Start);
                    }
                    _ => return Err(ReindeerMazeErr::UnrecognisedMazeChar),
                }
            }
            grid.push(maze_row);
        }

        if reindeer_pos.is_none() {
            return Err(ReindeerMazeErr::NoStartFound);
        }
        let reindeer_pos = reindeer_pos.unwrap();
        if !end_found {
            return Err(ReindeerMazeErr::NoEndFound);
        }

        Ok(Self { grid, reindeer_pos })
    }
}

impl ReindeerMaze {
    fn path_cost(visited: &Vec<Point>) -> Option<usize> {
        if visited.is_empty() {
            return None;
        }

        let mut current_direction = Direction::Right;
        let mut cost = 0;
        let mut prev_point = visited.first().expect("Empty visited path");
        for point in visited.iter().skip(1) {
            if let Some(dir) = prev_point.direction_of(point) {
                if dir == current_direction {
                    cost += 1;
                } else if dir == current_direction.right90() {
                    cost += 1001;
                    current_direction = current_direction.right90();
                } else if dir == current_direction.left90() {
                    cost += 1001;
                    current_direction = current_direction.left90();
                }
            }
            prev_point = point;
        }

        Some(cost)
    }

    fn possible_points(&self, source: &Point, path: &Vec<Point>) -> Vec<Point> {
        Direction::horiz_and_vert()
            .into_iter()
            .filter_map(|d| {
                let next_point = source.add(&d.to_point());
                if path.contains(&next_point) {
                    return None;
                }
                let contents = &self.grid[next_point.0 as usize][next_point.1 as usize];
                match contents {
                    MazeCell::Empty | MazeCell::End => Some(next_point),
                    _ => None,
                }
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
                if let Some(cost) = ReindeerMaze::path_cost(&candidate) {
                    min_cost = min_cost
                        .or(Some(usize::MAX))
                        .map(|x| x.min(cost));
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
        let maze = ReindeerMaze::from_str(puzzle).unwrap();
        assert_eq!(maze.part_a(), Some(2003));
    }

    #[test]
    fn test_test_small_txt() {
        let puzzle = include_str!("../puzzle/test_small.txt");
        let maze = ReindeerMaze::from_str(puzzle).unwrap();
        assert_eq!(maze.part_a(), Some(7036));
    }

    #[test]
    fn test_test_large_txt() {
        let puzzle = include_str!("../puzzle/test_large.txt");
        let maze = ReindeerMaze::from_str(puzzle).unwrap();
        assert_eq!(maze.part_a(), Some(11048));
    }
}

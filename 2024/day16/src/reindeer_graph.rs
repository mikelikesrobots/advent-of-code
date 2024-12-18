use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use pathfinding::{directed::yen::yen, prelude::dijkstra};
use util::{direction::Direction, point::Point};

use crate::{maze_cell::MazeCell, reindeer_maze_err::ReindeerMazeErr};

type FacingPoint = (Point, Direction);
type ReachableNodeWithCost = (FacingPoint, usize);

#[derive(Debug)]
pub struct ReindeerGraph {
    graph: HashMap<FacingPoint, Vec<ReachableNodeWithCost>>,
    start: Point,
    end: Point,
}

impl FromStr for ReindeerGraph {
    type Err = ReindeerMazeErr;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        // Get a grid of all the points
        let mut end_pos = None;
        let mut start_pos = None;
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
                        end_pos = Some(point);
                        maze_row.push(MazeCell::End);
                    }
                    '.' => maze_row.push(MazeCell::Empty),
                    'S' => {
                        start_pos = Some(point);
                        maze_row.push(MazeCell::Start);
                    }
                    _ => return Err(ReindeerMazeErr::UnrecognisedMazeChar),
                }
            }
            grid.push(maze_row);
        }

        if start_pos.is_none() {
            return Err(ReindeerMazeErr::NoStartFound);
        }
        let start_pos = start_pos.unwrap();
        if end_pos.is_none() {
            return Err(ReindeerMazeErr::NoEndFound);
        }
        let end_pos = end_pos.unwrap();

        // Build set of junctions
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

        let mut graph = HashMap::new();
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if *cell == MazeCell::Wall {
                    continue;
                }
                let point = Point(row_idx as i32, col_idx as i32);
                if !junctions.contains(&point) {
                    continue;
                }

                for dir in Direction::horiz_and_vert() {
                    let facing_point = (point, dir);
                    let mut reachable = vec![];

                    // Sanity check that left and right are not walls
                    let right_point = point.add(&dir.right90().to_point());
                    let right_contents = &grid[right_point.0 as usize][right_point.1 as usize];
                    if *right_contents != MazeCell::Wall {
                        reachable.push(((point, dir.right90()), 1000));
                    }
                    let left_point = point.add(&dir.left90().to_point());
                    let left_contents = &grid[left_point.0 as usize][left_point.1 as usize];
                    if *left_contents != MazeCell::Wall {
                        reachable.push(((point, dir.left90()), 1000));
                    }

                    let mut next = point.add(&dir.to_point());
                    // If a wall or the end, then stop there
                    let next_cell = &grid[next.0 as usize][next.1 as usize];
                    match next_cell {
                        MazeCell::Wall | MazeCell::Start => (),
                        MazeCell::End => {reachable.push(((next, dir), 1));},
                        MazeCell::Empty => {
                            let mut acc = 1;
                            while !junctions.contains(&next) {
                                next = next.add(&dir.to_point());
                                acc += 1;
                            }
                            reachable.push(((next, dir), acc));
                        }
                    }
                    graph.insert(facing_point, reachable);
                }
            }
        }

        Ok(ReindeerGraph {
            graph,
            start: start_pos,
            end: end_pos,
        })
    }
}

impl ReindeerGraph {
    fn successors(&self, node: &FacingPoint) -> Vec<ReachableNodeWithCost> {
        self.graph.get(node).unwrap().to_vec()
    }

    fn expand(path: &[FacingPoint]) -> Vec<Point> {
        let mut expanded = vec![];
        if path.is_empty() {
            return expanded;
        }

        let (mut prev_point, mut dir) = path.first().expect("Empty visited path");
        expanded.push(prev_point);
        for (next_point, next_dir) in path.iter().skip(1) {
            while prev_point != *next_point {
                prev_point = prev_point.add(&dir.to_point());
                expanded.push(prev_point);
            }
            dir = *next_dir;
        }

        expanded
    }

    pub fn part_a(&self) -> Option<usize> {
        let paths = dijkstra(
            &(self.start, Direction::Right),
            |p| self.successors(p),
            |&p| p.0 == self.end
        );
        paths
            .iter()
            // .filter_map(|path| ReindeerGraph::path_cost(&path.0))
            .map(|path| path.1)
            .min()
    }

    pub fn part_b(&self) -> Option<usize> {
        let mut shortest_paths: Option<Vec<(Vec<FacingPoint>, usize)>> = None;
        for path_count in [5, 10, 20, 30, 40, 50] {
            let paths = yen(
                &(self.start, Direction::Right),
                |p| self.successors(p),
                |&p| p.0 == self.end,
                path_count,
            );
            if paths.is_empty() {
                return None;
            }
            let shortest_cost = paths[0].1;
            let only_shortest: Vec<_> = paths
                .iter()
                .take_while(|(_, cost)| *cost == shortest_cost)
                .map(|(v, c)| (v.to_vec(), *c))
                .collect();
            if only_shortest.len() != paths.len() {
                shortest_paths = Some(only_shortest);
                break;
            }
        }

        let shortest_paths = shortest_paths.as_ref()?;

        let mut tiles_traversed: HashSet<Point> = HashSet::new();
        for (path, _) in shortest_paths {
            tiles_traversed.extend(ReindeerGraph::expand(path));
        }

        Some(tiles_traversed.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_very_small_txt() {
        let puzzle = include_str!("../puzzle/test_very_small.txt");
        let maze = ReindeerGraph::from_str(puzzle).unwrap();
        assert_eq!(maze.part_a(), Some(2003));
    }

    #[test]
    fn test_test_smaller_txt() {
        let puzzle = include_str!("../puzzle/test_smaller.txt");
        let maze = ReindeerGraph::from_str(puzzle).unwrap();
        assert_eq!(maze.part_a(), Some(2005));
    }

    #[test]
    fn test_test_small_txt_part_a() {
        let puzzle = include_str!("../puzzle/test_small.txt");
        let maze = ReindeerGraph::from_str(puzzle).unwrap();
        assert_eq!(maze.part_a(), Some(7036));
    }

    #[test]
    fn test_test_large_txt_part_a() {
        let puzzle = include_str!("../puzzle/test_large.txt");
        let maze = ReindeerGraph::from_str(puzzle).unwrap();
        assert_eq!(maze.part_a(), Some(11048));
    }

    #[test]
    fn test_test_small_txt_part_b() {
        let puzzle = include_str!("../puzzle/test_small.txt");
        let maze = ReindeerGraph::from_str(puzzle).unwrap();
        assert_eq!(maze.part_b(), Some(45));
    }

    #[test]
    fn test_test_large_txt_part_b() {
        let puzzle = include_str!("../puzzle/test_large.txt");
        let maze = ReindeerGraph::from_str(puzzle).unwrap();
        assert_eq!(maze.part_b(), Some(64));
    }
}

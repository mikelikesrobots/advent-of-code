use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};
use util::{direction::Direction, point::Point};

use crate::warehouse_err::{WarehouseErr, WarehouseMoveErr};

#[derive(Clone, Debug, PartialEq)]
enum WarehouseCell {
    Wall,
    LeftObject,
    RightObject,
    Robot,
}

#[derive(Clone, Debug)]
pub struct WideWarehouse {
    contents: HashMap<Point, WarehouseCell>,
    robot_pos: Point,
    robot_program: Vec<Direction>,
    width: usize,
    height: usize,
}

impl FromStr for WideWarehouse {
    type Err = WarehouseErr;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let mut contents = HashMap::new();
        let mut robot_pos: Option<Point> = None;
        let mut robot_program = vec![];

        let grid_lines: Vec<&str> = puzzle.lines().take_while(|l| !l.is_empty()).collect();
        if grid_lines.is_empty() {
            return Err(WarehouseErr::NoContentsFound);
        }
        let height = grid_lines.len() - 2;
        let width = (grid_lines[0].len() - 2) * 2;
        for (row_idx, row) in grid_lines.iter().enumerate() {
            if row_idx == 0 || (row_idx == height + 1) {
                continue;
            }
            for (col_idx, cell) in row.chars().enumerate() {
                if col_idx == 0 || (col_idx == width + 1) {
                    continue;
                }
                let left_point = Point((row_idx - 1) as i32, ((col_idx - 1) * 2) as i32);
                let right_point = Point((row_idx - 1) as i32, (((col_idx - 1) * 2) + 1) as i32);

                match cell {
                    '#' => {
                        contents.insert(left_point, WarehouseCell::Wall);
                        contents.insert(right_point, WarehouseCell::Wall);
                    }
                    '.' => (),
                    'O' => {
                        contents.insert(left_point, WarehouseCell::LeftObject);
                        contents.insert(right_point, WarehouseCell::RightObject);
                    }
                    '@' => {
                        contents.insert(left_point, WarehouseCell::Robot);
                        robot_pos = Some(left_point);
                    }
                    _ => return Err(WarehouseErr::UnrecognisedCell),
                }
            }
        }

        let directions_str = puzzle
            .lines()
            .skip_while(|l| !l.is_empty())
            .collect::<String>();
        for c in directions_str.chars() {
            let dir = match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '>' => Direction::Right,
                '<' => Direction::Left,
                _ => return Err(WarehouseErr::UnrecognisedDirection),
            };
            robot_program.push(dir);
        }

        if contents.is_empty() {
            return Err(WarehouseErr::NoContentsFound);
        }
        if robot_pos.is_none() {
            return Err(WarehouseErr::NoRobotFound);
        }
        if robot_program.is_empty() {
            return Err(WarehouseErr::NoProgramFound);
        }

        Ok(Self {
            contents,
            robot_pos: robot_pos.unwrap(),
            robot_program,
            width,
            height,
        })
    }
}

impl WideWarehouse {
    fn print_grid(&self) {
        let mut grid = "".to_string();
        for _ in 0..self.width + 4 {
            grid += "#";
        }
        grid += "\n";

        for row_idx in 0..self.height {
            grid += "##";
            for col_idx in 0..self.width {
                let point = Point(row_idx as i32, col_idx as i32);
                grid += match self.contents.get(&point) {
                    None => ".",
                    Some(WarehouseCell::LeftObject) => "[",
                    Some(WarehouseCell::RightObject) => "]",
                    Some(WarehouseCell::Wall) => "#",
                    Some(WarehouseCell::Robot) => "@",
                };
            }
            grid += "##\n";
        }

        for _ in 0..self.width + 4 {
            grid += "#";
        }
        grid += "\n";

        println!("{}", grid);
    }

    fn gps_sum(&self) -> usize {
        self.contents
            .iter()
            .filter_map(|(point, cell)| {
                if *cell != WarehouseCell::LeftObject {
                    return None;
                }

                let y_gps = ((point.0 as usize) + 1) * 100;
                let x_gps = (point.1 as usize) + 2;
                Some(x_gps + y_gps)
            })
            .sum()
    }

    fn in_bounds(&self, point: &Point) -> bool {
        point.0 >= 0
            && point.1 >= 0
            && point.0 < (self.height as i32)
            && point.1 < (self.width as i32)
    }

    fn build_move_stack(
        &self,
        point: &Point,
        direction: &Direction,
        visited: &mut HashSet<Point>,
    ) -> Vec<Result<(Point, Point), WarehouseMoveErr>> {
        let candidate = point.add(&direction.to_point());
        if !self.in_bounds(&candidate) {
            return vec![Err(WarehouseMoveErr::OutOfBounds)];
        }

        let mut move_box = |candidate: &Point, other: &Option<&WarehouseCell>| {
            if visited.contains(point) {
                return vec![];
            }
            let current= self.contents.get(&point);
            let mut stack = vec![];
            visited.insert(*point);
            
            match other {
                None => stack.push(Ok((*point, *candidate))),
                Some(_) => {
                    stack.push(Ok((*point, *candidate)));
                    stack.append(&mut self.build_move_stack(&candidate, direction, visited));
                }
            }

            let other_side_of_box = match current {
                Some(WarehouseCell::LeftObject) => point.add(&Direction::Right.to_point()),
                Some(WarehouseCell::RightObject) => point.add(&Direction::Left.to_point()),
                _ => return vec![],
            };
            if !visited.contains(&other_side_of_box) {
                stack.append(&mut self.build_move_stack(&other_side_of_box, direction, visited));
            }
            stack
        };

        match (self.contents.get(&point), self.contents.get(&candidate)) {
            // I should never be empty or a wall at the current point, or be anything moving into the robot.
            (None, _) | (Some(WarehouseCell::Wall), _) | (_, Some(WarehouseCell::Robot)) => {
                vec![Err(WarehouseMoveErr::InvalidPointMoved)]
            }
            // If the next point is a wall, object cannot move.
            (_, Some(WarehouseCell::Wall)) => vec![Err(WarehouseMoveErr::MoveObstructed)],
            // A robot moving into empty space is okay. Ignore visited
            (Some(WarehouseCell::Robot), None) => vec![Ok((*point, candidate))],
            // Either side of a box needs extra logic. Test the other side of the box too.
            (Some(WarehouseCell::LeftObject), other) => {
                move_box(&candidate, &other)
            },
            (Some(WarehouseCell::RightObject), other) => {
                move_box(&candidate, &other)
            },
            (Some(WarehouseCell::Robot), Some(WarehouseCell::LeftObject))
            | (Some(WarehouseCell::Robot), Some(WarehouseCell::RightObject)) => {
                visited.insert(*point);
                let mut stack = vec![Ok((*point, candidate))];
                stack.append(&mut self.build_move_stack(&candidate, direction, visited));
                stack
            }
        }
    }

    fn move_robot_and_boxes(&mut self, direction: &Direction) -> Result<(), WarehouseMoveErr> {
        // Transform moves to Vec<(Point, Point)>
        let try_moves: Result<Vec<_>, _> = self
            .build_move_stack(&self.robot_pos, direction, &mut HashSet::new())
            .into_iter()
            .collect();
        let moves = try_moves?;
        match moves.first() {
            Some(x) => {
                self.robot_pos = x.1;
            }
            None => return Err(WarehouseMoveErr::UnexpectedlyEmptyMoveList),
        }
        let mut new_contents = self.contents.clone();

        for (old_pos, _) in moves.iter() {
            let _ = new_contents.remove(old_pos);
        }
        for (old_pos, new_pos) in moves.iter() {
            if let Some(obj) = self.contents.get(old_pos) {
                new_contents.insert(*new_pos, obj.clone());
            }
        }
        self.contents = new_contents;
        Ok(())
    }

    pub fn part_b(&mut self) -> usize {
        let robot_program = self.robot_program.clone();
        for d in robot_program.iter() {
            let _ = self.move_robot_and_boxes(d);
        }

        self.gps_sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_small_txt() {
        let puzzle = include_str!("../puzzle/test_small_wide.txt");
        let mut warehouse = WideWarehouse::from_str(puzzle).unwrap();
        warehouse.print_grid();
        assert_eq!(105, warehouse.part_b());
        assert!(false);
    }

    #[test]
    fn test_test_large_txt() {
        let puzzle = include_str!("../puzzle/test_large.txt");
        let mut warehouse = WideWarehouse::from_str(puzzle).unwrap();
        warehouse.print_grid();
        println!("{:?}", warehouse);
        assert_eq!(9021, warehouse.part_b());
    }
}

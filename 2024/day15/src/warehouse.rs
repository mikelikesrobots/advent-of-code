use std::{collections::HashMap, str::FromStr};

use util::{direction::Direction, point::Point};

use crate::warehouse_err::WarehouseErr;

#[derive(Clone, Debug, PartialEq)]
enum WarehouseCell {
    Wall,
    Object,
}

#[derive(Clone, Debug)]
pub struct Warehouse {
    contents: HashMap<Point, WarehouseCell>,
    robot: Point,
    robot_program: Vec<Direction>,
    width: usize,
    height: usize,
}

impl FromStr for Warehouse {
    type Err = WarehouseErr;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let mut contents = HashMap::new();
        let mut robot: Option<Point> = None;
        let mut robot_program = vec![];

        let grid_lines: Vec<&str> = puzzle.lines().take_while(|l| !l.is_empty()).collect();
        if grid_lines.is_empty() {
            return Err(WarehouseErr::NoContentsFound);
        }
        let height = grid_lines.len() - 2;
        let width = grid_lines[0].len() - 2;
        for (row_idx, row) in grid_lines.iter().enumerate() {
            if row_idx == 0 || (row_idx == height + 1) {
                continue;
            }
            for (col_idx, cell) in row.chars().enumerate() {
                if col_idx == 0 || (col_idx == width + 1) {
                    continue;
                }
                let point = Point((row_idx - 1) as i32, (col_idx - 1) as i32);
                match cell {
                    '#' => {
                        contents.insert(point, WarehouseCell::Wall);
                    },
                    '.' => (),
                    'O' => {
                        contents.insert(point, WarehouseCell::Object);
                    },
                    '@' => {
                        robot = Some(point);
                    },
                    _ => return Err(WarehouseErr::UnrecognisedCell),
                }
            }
        }
 
        let directions_str = puzzle.lines().skip_while(|l| !l.is_empty()).collect::<String>();
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
        if robot.is_none() {
            return Err(WarehouseErr::NoRobotFound);
        }
        if robot_program.is_empty() {
            return Err(WarehouseErr::NoProgramFound);
        }

        Ok(Warehouse { contents, robot: robot.unwrap(), robot_program, width, height })
    }
}

impl Warehouse {
    fn gps_sum(&self) -> usize {
        self.contents.iter().filter_map(|(point, cell)| {
            if *cell != WarehouseCell::Object {
                return None;
            }
            let y_gps = (point.0 as usize + 1) * 100; 
            let x_gps = point.1 as usize + 1;
            Some(x_gps + y_gps)
        }).sum()
    }

    fn in_bounds(&self, point: &Point) -> bool {
        point.0 >= 0 && point.1 >= 0 && point.0 < (self.height as i32) && point.1 < (self.width as i32)
    }

    fn move_object(&mut self, point: &Point, direction: &Direction) -> bool {
        let candidate = point.add(&direction.to_point());
        if !self.in_bounds(&candidate) {
            return false;
        }

        match self.contents.get(&candidate) {
            None => {
                true
            },
            Some(WarehouseCell::Object) => {
                if self.move_object(&candidate, direction) {
                    if let Some(obj) = self.contents.remove(&candidate) {
                        self.contents.insert(candidate.add(&direction.to_point()), obj);
                    }
                    return true;
                } else {
                    return false;
                }
            },
            Some(WarehouseCell::Wall) => false,
        }
    }

    fn print_grid(&self) {
        let mut grid = "".to_string();
        for _ in 0..self.width + 2 {
            grid += "#";
        }
        grid += "\n";

        for row_idx in 0..self.height {
            grid += "#";
            for col_idx in 0..self.width {
                let point = Point(row_idx as i32, col_idx as i32);
                if self.robot == point {
                    grid += "@";
                    continue;
                }
                grid += match self.contents.get(&point) {
                    None => ".",
                    Some(WarehouseCell::Object) => "O",
                    Some(WarehouseCell::Wall) => "#",
                };
            }
            grid += "#\n";
        }

        for _ in 0..self.width + 2 {
            grid += "#";
        }
        grid += "\n";

        println!("{}", grid);
    }

    pub fn part_a(&mut self) -> usize {
        let robot_program = self.robot_program.clone();
        // self.print_grid();
        for d in robot_program.iter() {
            let robot_pos_candidate = self.robot.add(&d.to_point());
            if !self.in_bounds(&robot_pos_candidate) {
                continue;
            }

            match self.contents.get(&robot_pos_candidate) {
                None => self.robot = robot_pos_candidate,
                Some(WarehouseCell::Object) => {
                    if self.move_object(&robot_pos_candidate, d) {
                        if let Some(obj) = self.contents.remove(&robot_pos_candidate) {
                            self.contents.insert(robot_pos_candidate.add(&d.to_point()), obj);
                        }
                        self.robot = robot_pos_candidate;
                    }
                },
                Some(WarehouseCell::Wall) => (),
            }

            // println!("Direction: {:?}", d);
            // self.print_grid();
            // println!("");
        }

        self.gps_sum()
    }
}

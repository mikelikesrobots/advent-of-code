use std::{collections::HashSet, str::FromStr};
use util::{direction::Direction, point::Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FacingPoint {
    pos: Point,
    dir: Direction,
}

#[derive(Debug)]
struct GuardedLab {
    obstacles: HashSet<Point>,
    guard_start: FacingPoint,
    height: usize,
    width: usize,
}

#[derive(Debug)]
enum GuardedLabErr {
    UnrecognisedCharacter,
    GuardNotFound,
}
impl FromStr for GuardedLab {
    type Err = GuardedLabErr;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let mut obstacles = HashSet::new();
        let mut guard: Option<FacingPoint> = None;
        let mut height = 0;
        let mut width = 0;
        for (row_idx, row) in puzzle.lines().enumerate() {
            for (col_idx, cell) in row.chars().enumerate() {
                let p = Point(row_idx as i32, col_idx as i32);
                match cell {
                    '#' => _ = obstacles.insert(p),
                    '^' => {
                        guard = Some(FacingPoint {
                            pos: p,
                            dir: Direction::Up,
                        });
                    }
                    '>' => {
                        guard = Some(FacingPoint {
                            pos: p,
                            dir: Direction::Right,
                        });
                    }
                    '<' => {
                        guard = Some(FacingPoint {
                            pos: p,
                            dir: Direction::Left,
                        });
                    }
                    'v' => {
                        guard = Some(FacingPoint {
                            pos: p,
                            dir: Direction::Down,
                        });
                    }
                    '.' => continue,
                    _ => return Err(GuardedLabErr::UnrecognisedCharacter),
                }
                width = width.max(col_idx + 1);
            }
            height = height.max(row_idx + 1);
        }

        let guard_start = match guard {
            Some(g) => g,
            None => return Err(GuardedLabErr::GuardNotFound),
        };

        Ok(GuardedLab {
            obstacles,
            guard_start,
            height,
            width,
        })
    }
}

impl GuardedLab {

    fn in_bounds(&self, point: &Point) -> bool {
        point.0 >= 0 &&
            point.1 >= 0 &&
            point.0 < (self.height as i32) &&
            point.1 < (self.width as i32)
    }

    fn get_guard_path(&self) -> HashSet<Point> {
        let mut travelled: HashSet<Point> = HashSet::new();
        let mut guard = self.guard_start;
        
        while self.in_bounds(&guard.pos) {
            travelled.insert(guard.pos);
            let next_pos = guard.pos.add(&guard.dir.to_point());
            if self.obstacles.contains(&next_pos) {
                guard = FacingPoint { pos: guard.pos, dir: guard.dir.right90()};
                continue;
            }

            guard = FacingPoint { pos: next_pos, dir: guard.dir };
        }
        travelled
    }

    fn causes_loop(&self, obstacle: &Point) -> bool {
        let mut travelled: HashSet<FacingPoint> = HashSet::new();
        let mut guard = self.guard_start;

        while self.in_bounds(&guard.pos) {
            travelled.insert(guard);
            let next_pos = guard.pos.add(&guard.dir.to_point());
            if self.obstacles.contains(&next_pos) || *obstacle == next_pos {
                guard = FacingPoint { pos: guard.pos, dir: guard.dir.right90()};
                if travelled.contains(&guard) {
                    return true;
                }
                continue;
            }

            guard = FacingPoint { pos: next_pos, dir: guard.dir };
            if travelled.contains(&guard) {
                return true;
            }
        }
        false
    }

    fn part_a(&self) -> usize {
        let travelled = self.get_guard_path();
        travelled.len()
    }

    fn part_b(&self) -> usize {
        self.get_guard_path()
            .iter()
            .filter(|pos| self.causes_loop(pos))
            .count()
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let lab = GuardedLab::from_str(puzzle).expect("Unable to parse GuardedLab");
    println!("Part A: {}", lab.part_a());
    println!("Part B: {}", lab.part_b());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ok() {
        let input = "#.#\n>.#";
        let expected_obstacles = {
            let mut obs = HashSet::new();
            obs.insert(Point(0, 0));
            obs.insert(Point(0, 2));
            obs.insert(Point(1, 2));
            obs
        };
        let expected_guard = FacingPoint {
            pos: Point(1, 0),
            dir: Direction::Right,
        };
        let expected_height = 2;
        let expected_width = 3;

        let lab = GuardedLab::from_str(input).unwrap();
        println!("{:?}", lab);
        assert_eq!(lab.guard_start, expected_guard);
        assert_eq!(lab.width, expected_width);
        assert_eq!(lab.height, expected_height);
        assert_eq!(lab.obstacles, expected_obstacles);
    }

    #[test]
    fn test_test_txt_part_a() {
        let input = include_str!("../puzzle/test.txt");
        let lab = GuardedLab::from_str(input).unwrap();
        assert_eq!(lab.part_a(), 41);
    }

    #[test]
    fn test_test_txt_part_b() {
        let input = include_str!("../puzzle/test.txt");
        let lab = GuardedLab::from_str(input).unwrap();
        assert_eq!(lab.part_b(), 6);
    }

    #[test]
    fn test_guard_is_in_loop() {
        let input = ".#.\n#^#\n...";
        let new_obs = Point(2, 1);
        let lab = GuardedLab::from_str(input).unwrap();
        assert!(lab.causes_loop(&new_obs));
    }
}

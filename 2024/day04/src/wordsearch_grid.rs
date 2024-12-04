use std::str::FromStr;
use crate::direction::Direction;
use crate::point::Point;

#[derive(Debug)]
pub struct WordsearchGrid {
    grid: Vec<Vec<char>>,
}

#[derive(Debug)]
pub struct WordsearchGridErr {}
impl FromStr for WordsearchGrid {
    type Err = WordsearchGridErr;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let grid = puzzle.lines().map(|line| line.chars().collect()).collect();
        Ok(WordsearchGrid { grid })
    }
}

impl WordsearchGrid {
    fn check(&self, p: &Point, c: char) -> bool {
        if p.0 < 0 || p.0 >= self.grid.len() as i32 || p.1 < 0 || p.1 >= self.grid[0].len() as i32 {
            return false;
        }
        self.grid[p.0 as usize][p.1 as usize] == c
    }

    fn find_xmas(&self, x_point: &Point, dir: &Direction) -> bool {
        let point_diff = dir.to_point();
        let mut check_point = x_point.add(&point_diff);
        if !self.check(&check_point, 'M') {
            return false;
        }
        check_point = check_point.add(&point_diff);
        if !self.check(&check_point, 'A') {
            return false;
        }
        check_point = check_point.add(&point_diff);
        self.check(&check_point, 'S')
    }

    fn count_xmas(&self, x_point: &Point) -> usize {
        Direction::all_directions()
            .iter()
            .filter(|dir| self.find_xmas(x_point, dir))
            .count()
    }

    fn check_x_mas(&self, a_point: &Point) -> bool {
        Direction::corners()
            .iter()
            .filter(|d| {
                let m_ok = self.check(&a_point.add(&d.to_point()), 'M');
                let s_ok = self.check(&a_point.add(&d.opposite().to_point()), 'S');
                m_ok && s_ok
            })
            .count() == 2
    }

    pub fn part_a(&self) -> usize {
        let mut xmas_count = 0;

        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col == 'X' {
                    xmas_count += self.count_xmas(&Point(row_idx as i32, col_idx as i32));
                }
            }
        }

        xmas_count
    }

    pub fn part_b(&self) -> usize {
        let mut x_mas_count = 0;

        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col == 'A' && self.check_x_mas(&Point(row_idx as i32, col_idx as i32)) {
                    x_mas_count += 1;
                }
            }
        }

        x_mas_count
    }
}

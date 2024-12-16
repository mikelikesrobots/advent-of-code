use std::{
    cmp::Ordering, collections::{HashMap, HashSet}, fs::File, num::ParseIntError
};
use std::{thread, time};

use bmp_monochrome::Bmp;
use util::point::Point;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Robot {
    pos: Point,
    vel: Point,
}

impl Robot {
    fn new_from_strs(p_x: &str, p_y: &str, v_x: &str, v_y: &str) -> Result<Robot, ParseIntError> {
        let p_x = p_x.parse()?;
        let p_y = p_y.parse()?;
        let v_x = v_x.parse()?;
        let v_y = v_y.parse()?;
        let pos = Point(p_x, p_y);
        let vel = Point(v_x, v_y);
        Ok(Robot { pos, vel })
    }

    fn pos_after(&self, steps: i64, width: i64, height: i64) -> Point {
        let mut x: i64 = (self.pos.0 as i64) + steps * (self.vel.0 as i64);
        let mut y: i64 = (self.pos.1 as i64) + steps * (self.vel.1 as i64);

        while x < 0 {
            x += width;
        }
        x = x % width;
        while y < 0 {
            y += height;
        }
        y = y % height;

        Point(x as i32, y as i32)
    }

    fn tick(&mut self, width: i32, height: i32) {
        self.pos = self.pos_after(1, width as i64, height as i64);
    }

    fn tick_n(&mut self, steps: usize, width: i32, height: i32) {
        self.pos = self.pos_after(steps as i64, width as i64, height as i64);
    }
}

#[derive(Debug)]
struct RestroomSimulation {
    robots: Vec<Robot>,
    width: usize,
    height: usize,
}

impl RestroomSimulation {
    fn from(
        puzzle: &str,
        width: usize,
        height: usize,
    ) -> Result<RestroomSimulation, anyhow::Error> {
        let re = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")?;
        let robots = re
            .captures_iter(puzzle)
            .map(|c| c.extract())
            .filter_map(|(_, [p_x, p_y, v_x, v_y])| Robot::new_from_strs(p_x, p_y, v_x, v_y).ok())
            .collect::<Vec<_>>();

        Ok(RestroomSimulation {
            robots,
            width,
            height,
        })
    }

    fn safety_after_steps(&self, steps: usize) -> usize {
        let robots = self
            .robots
            .iter()
            .map(|r| r.pos_after(steps as i64, self.width as i64, self.height as i64))
            .collect::<Vec<Point>>();
        RestroomSimulation::count_in_quadrants(&robots, self.width, self.height)
    }

    fn count_in_quadrants(robot_positions: &[Point], width: usize, height: usize) -> usize {
        let mut top_left = 0;
        let mut top_right = 0;
        let mut btm_left = 0;
        let mut btm_right = 0;
        let half_width = (width / 2) as i32;
        let half_height = (height / 2) as i32;
        for pos in robot_positions {
            match (pos.0.cmp(&half_width), pos.1.cmp(&half_height)) {
                (Ordering::Equal, _) | (_, Ordering::Equal) => (),
                (Ordering::Less, Ordering::Less) => top_left += 1,
                (Ordering::Greater, Ordering::Less) => top_right += 1,
                (Ordering::Less, Ordering::Greater) => btm_left += 1,
                (Ordering::Greater, Ordering::Greater) => btm_right += 1,
            }
        }

        top_left * top_right * btm_left * btm_right
    }

    fn part_a(&self) -> usize {
        self.safety_after_steps(100)
    }

    fn tick(&mut self) {
        self.robots
            .iter_mut()
            .for_each(|r| r.tick(self.width as i32, self.height as i32));
    }

    fn tick_n(&mut self, ticks: usize) {
        self.robots
            .iter_mut()
            .for_each(|r| r.tick_n(ticks, self.width as i32, self.height as i32));
    }

    fn to_diagram(&self) -> Result<Bmp, anyhow::Error> {
        let robot_positions = {
            let mut robot_positions = HashSet::new();
            self.robots.iter().for_each(|r| {
                robot_positions.insert(&r.pos);
            });
            robot_positions
        };

        let mut rows = vec![];

        for col_idx in 0..self.width {
            let mut row = vec![];
            for row_idx in 0..self.height {
                let pos = Point(col_idx as i32, row_idx as i32);
                row.push(robot_positions.contains(&pos));
            }
            rows.push(row);
        }

        Ok(Bmp::new(rows)?)
    }

    fn write_diagram(&self, step_count: usize) -> Result<(), anyhow::Error> {
        let filename = format!("trees/tree_{}.bmp", step_count);
        let bmp = self.to_diagram()?;
        bmp.write(File::create(filename)?)?;
        Ok(())
    }

    fn read_easter_egg() -> Result<Bmp, anyhow::Error> {
        Ok(Bmp::read(File::open("puzzle/easter_egg.bmp")?)?)
    }

    fn part_b(&mut self) -> usize {

        // The following code eventually results in 7083
        let mut steps = 0;
        let correct = RestroomSimulation::read_easter_egg().expect("Could not read correct solution");

        while self.to_diagram().expect("Could not convert to diagram") != correct {
            steps += 1;
            self.tick();
        }
        steps

        // Code to write diagrams to disk
        // loop {
        //     self.write_diagram(steps).expect("Could not write diagram");
        //     steps += 1;
        //     println!("{}", steps);
        //     self.tick();
        // }
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let mut sim = RestroomSimulation::from(puzzle, 101, 103).unwrap();
    println!("Part A: {}", sim.part_a());
    println!("Part B: {}", sim.part_b());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_test_txt() {
        let puzzle = include_str!("../puzzle/test.txt");
        let sim = RestroomSimulation::from(puzzle, 11, 7).unwrap();
        println!("{:?}", sim);
        assert_eq!(sim.part_a(), 12);
    }
}

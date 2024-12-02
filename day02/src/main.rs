use std::str::FromStr;

#[derive(Debug)]
struct ReportGrid {
    grid: Vec<Vec<i32>>,
}

#[derive(Debug)]
enum ReportGridError {}

impl FromStr for ReportGrid {
    type Err = ReportGridError;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let mut grid = vec![];
        for line in puzzle.lines() {
            let report = line
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            grid.push(report);
        }

        Ok(ReportGrid { grid })
    }
}

fn is_safe(report: &[i32]) -> bool {
    let differences: Vec<i32> = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();

    // The levels are either all increasing or all decreasing.
    let positive = differences[0] > 0;
    let all_one_way = differences.iter().all(|x| (*x > 0) == positive);
    if !all_one_way {
        return false;
    }

    // Any two adjacent levels differ by at least one and at most three.
    let level_changes_safe = differences
        .iter()
        .map(|x| x.abs())
        .all(|x| (1..=3).contains(&x));

    level_changes_safe
}

fn is_safe_dampened(report: &[i32]) -> bool {
        // Check base case
        if is_safe(report) {
            return true;
        }
        // Try removing each element in turn until a successful case is found
        for idx in 0..report.len() {
            let report_clone = {
                let mut clone = report.to_vec();
                clone.remove(idx);
                clone
            };
            if is_safe(&report_clone) {
                return true;
            }
        }
    
        false
}

impl ReportGrid {
    fn part_a(&self) -> usize {
        let count = self.grid.iter().filter(|report| is_safe(report)).count();

        count
    }

    fn part_b(&self) -> usize {
        let count = self
            .grid
            .iter()
            .filter(|report| is_safe_dampened(report))
            .count();

        count
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let grid = ReportGrid::from_str(puzzle).expect("Unable to parse input puzzle");
    println!("Part A: {}", grid.part_a());
    println!("Part B: {}", grid.part_b());
}

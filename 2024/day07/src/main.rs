use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
struct CalibrationSet {
    calibrations: HashMap<i64, Vec<i64>>,
}

#[derive(Debug)]
enum CalibrationSetErr {
    KeyMissing,
    KeyParseFailed,
    ValuesMissing,
}
impl FromStr for CalibrationSet {
    type Err = CalibrationSetErr;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let mut calibrations = HashMap::new();
        for line in puzzle.lines() {
            let key = line
                .split(":")
                .nth(0)
                .ok_or(CalibrationSetErr::KeyMissing)?
                .parse()
                .map_err(|_| CalibrationSetErr::KeyParseFailed)?;
            let values = line
                .split(":")
                .nth(1)
                .ok_or(CalibrationSetErr::ValuesMissing)?
                .split(" ")
                .filter_map(|x| x.parse().ok())
                .collect();
            calibrations.insert(key, values);
        }

        Ok(CalibrationSet { calibrations })
    }
}

impl CalibrationSet {
    fn combination_possible_no_concat(key: i64, vals: &[i64]) -> bool {
        let reversed = vals.iter().rev().copied().collect::<Vec<i64>>();
        CalibrationSet::all_combinations_no_concat(&reversed).contains(&key)
    }

    fn combination_possible_with_concat(key: i64, vals: &[i64]) -> bool {
        CalibrationSet::all_combinations_with_concat(key, vals).contains(&key)
    }

    fn all_combinations_with_concat(max_val: i64, vals: &[i64]) -> Vec<i64> {
        if vals.len() < 2 {
            return vals.to_vec();
        }
        let current_val = vals.last().unwrap();
        let sub_combinations = CalibrationSet::all_combinations_with_concat(max_val, &vals[0..vals.len() - 1]);

        let add_combinations: Vec<i64> = sub_combinations.iter().map(|x| x + current_val).collect();
        let mul_combinations: Vec<i64> = sub_combinations.iter().map(|x| x * current_val).collect();
        let concat_combinations: Vec<i64> = sub_combinations
            .iter()
            .filter_map(|x| (x.to_string() + &current_val.to_string()).parse::<i64>().ok())
            .collect();
        add_combinations
            .into_iter()
            .chain(mul_combinations)
            .chain(concat_combinations)
            .filter(|&x| x <= max_val)
            .collect()
    }

    fn all_combinations_no_concat(vals: &[i64]) -> Vec<i64> {
        if vals.len() == 1 {
            return vals.to_vec();
        }
        let sub_combinations = CalibrationSet::all_combinations_no_concat(&vals[1..]);

        let add_combinations = sub_combinations.iter().map(|x| x + vals[0]);
        let mul_combinations = sub_combinations.iter().map(|x| x * vals[0]);
        add_combinations.chain(mul_combinations).collect()
    }

    fn part_a(&self) -> usize {
        self.calibrations
            .iter()
            .filter(|(&key, vals)| CalibrationSet::combination_possible_no_concat(key, vals))
            .map(|(&key, _)| key)
            .sum::<i64>() as usize
    }

    fn part_b(&self) -> usize {
        self.calibrations
            .iter()
            .filter(|(&key, vals)| CalibrationSet::combination_possible_with_concat(key, vals))
            .map(|(&key, _)| key)
            .sum::<i64>() as usize
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let calibs = CalibrationSet::from_str(puzzle).unwrap();
    println!("Part A: {}", calibs.part_a());
    println!("Part B: {}", calibs.part_b());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_valid_line_no_concat() {
        assert!(CalibrationSet::combination_possible_no_concat(
            190,
            &[10, 19]
        ));
    }
    #[test]
    fn test_short_valid_line_with_concat() {
        assert!(CalibrationSet::combination_possible_with_concat(
            156,
            &[15, 6]
        ));
    }

    #[test]
    fn test_medium_valid_line_no_concat() {
        assert!(CalibrationSet::combination_possible_no_concat(
            3267,
            &[81, 40, 27]
        ));
    }

    #[test]
    fn test_medium_valid_line_with_concat() {
        assert!(CalibrationSet::combination_possible_with_concat(
            7290,
            &[6, 8, 6, 15]
        ));
    }

    #[test]
    fn test_harder_valid_line_no_concat() {
        assert!(CalibrationSet::combination_possible_no_concat(
            292,
            &[11, 6, 16, 20]
        ));
    }

    #[test]
    fn test_invalid_line_no_concat() {
        assert!(!CalibrationSet::combination_possible_no_concat(
            21037,
            &[9, 7, 18, 13]
        ));
    }

    #[test]
    fn test_test_txt_no_concat() {
        let puzzle = include_str!("../puzzle/test.txt");
        let set = CalibrationSet::from_str(puzzle).unwrap();
        assert_eq!(set.part_a(), 3749);
    }

    #[test]
    fn test_test_txt_with_concat() {
        let puzzle = include_str!("../puzzle/test.txt");
        let set = CalibrationSet::from_str(puzzle).unwrap();
        assert_eq!(set.part_b(), 11387);
    }
}

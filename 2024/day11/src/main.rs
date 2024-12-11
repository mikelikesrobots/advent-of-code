use std::{collections::HashMap, num::ParseIntError, str::FromStr};

struct PlutoStones {
    stones: Vec<u64>,
    cache: HashMap<(u64, usize), usize>,
}

impl FromStr for PlutoStones {
    type Err = ParseIntError;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let stones = puzzle
            .split_whitespace()
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()?;
        Ok(PlutoStones { stones, cache: HashMap::new() })
    }
}

impl PlutoStones {
    fn apply_rule(stone: u64) -> (u64, Option<u64>) {
        if stone == 0 {
            return (1, None);
        }
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            let n_chars = stone_str.len() / 2;
            let left_str: String = stone_str.chars().take(n_chars).collect();
            let right_str: String = stone_str.chars().skip(n_chars).take(n_chars).collect();
            let left = left_str
                .parse()
                .unwrap_or_else(|_| panic!("Could not parse string: {}", left_str));
            let right = right_str
                .parse()
                .unwrap_or_else(|_| panic!("Could not parse string: {}", right_str));
            return (left, Some(right));
        }

        (stone * 2024, None)
    }

    fn stones_after_iterations(&mut self, initial_stone: u64, iterations: usize) -> usize {
        let mut stones = vec![initial_stone];
        let mut acc = 0;

        for idx in 0..iterations {
            let mut next_stones = vec![];
            let iter_remaining = iterations - idx;
            for stone in stones {
                // Check if stone is cached with n iterations remaining
                if let Some(x) = self.cache.get(&(stone, iter_remaining)) {
                    acc += x;
                    continue;
                }

                let (left, right_opt) = PlutoStones::apply_rule(stone);
                next_stones.push(left);
                if let Some(right) = right_opt {
                    next_stones.push(right);
                }
            }
            stones = next_stones;

            self.cache.insert((initial_stone, idx + 1), stones.len());
        }

        acc + stones.len()
    }

    fn part_a(&mut self) -> usize {
        let stones = self.stones.clone();
        stones
            .iter()
            .map(|&stone| self.stones_after_iterations(stone, 25))
            .sum()
    }

    fn part_b(&mut self) -> usize {
        let stones = self.stones.clone();
        stones
            .iter()
            .map(|&stone| self.stones_after_iterations(stone, 75))
            .sum()
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let mut stones = PlutoStones::from_str(puzzle).expect("Could not parse puzzle input!");
    println!("Part A: {}", stones.part_a());
    println!("Part B: {}", stones.part_b());
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::PlutoStones;

    #[test]
    fn test_test_txt() {
        let puzzle = include_str!("../puzzle/test.txt");
        let mut stones = PlutoStones::from_str(puzzle).unwrap();
        assert_eq!(55312, stones.part_a());
    }
}

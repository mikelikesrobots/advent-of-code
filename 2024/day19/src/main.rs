use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use regex::Regex;

#[derive(Debug)]
struct TowelDesigns {
    towels: HashSet<String>,
    towel_counts: HashMap<String, usize>,
    designs: Vec<String>,
    max_towel_length: usize,
    towel_lengths: HashSet<usize>,
}

#[derive(Debug)]
enum TowelDesignParseErr {
    NoTowelsFound,
    NoDesignsFound,
}
impl FromStr for TowelDesigns {
    type Err = TowelDesignParseErr;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let towels: HashSet<String> = puzzle
            .lines()
            .nth(0)
            .ok_or(TowelDesignParseErr::NoTowelsFound)?
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        if towels.is_empty() {
            return Err(TowelDesignParseErr::NoTowelsFound);
        }
        let designs: Vec<String> = puzzle.lines().skip(2).map(|s| s.to_string()).collect();
        if designs.is_empty() {
            return Err(TowelDesignParseErr::NoDesignsFound);
        }

        let max_towel_length = towels.iter().map(|s| s.len()).max().unwrap_or(0);
        let towel_lengths = towels.iter().map(|s| s.len()).collect();

        Ok(TowelDesigns {
            towels,
            towel_counts: HashMap::new(),
            designs,
            max_towel_length,
            towel_lengths,
        })
    }
}

impl TowelDesigns {
    fn insert_cache(&mut self, value: &str) {
        self.towels.insert(value.to_string());
        self.max_towel_length = self.max_towel_length.max(value.len());
        self.towel_lengths.insert(value.len());
    }

    fn is_possible(&mut self, remaining_design: &str, cache: &str) -> bool {
        // println!("Is {} possible?", remaining_design);
        if self.towels.contains(remaining_design) {
            // let found_towel = cache.to_string() + remaining_design;
            // println!("Found towel: {}", found_towel);
            // self.insert_cache(&found_towel);
            // println!("Yes, {} is possible!", remaining_design);
            return true;
        }

        // println!("Remaining design: {}", remaining_design);
        let mut towel_lengths: Vec<usize> = self
            .towel_lengths
            .iter()
            .filter(|&&x| x < remaining_design.len())
            .copied()
            .collect();
        towel_lengths.sort_unstable();
        for idx in towel_lengths.iter().rev() {
            // println!("idx: {}", idx);
            let check_str = &remaining_design[0..*idx];
            // println!("Checking: {}", check_str);

            let leftover_design = &remaining_design[*idx..];
            let contained = self.towels.contains(check_str);
            if contained && self.is_possible(leftover_design, &(cache.to_string() + &check_str)) {
                let string_so_far = cache.to_string() + &check_str;
                // println!("Adding towels: {}, {}", string_so_far, leftover_design);
                self.towels.insert(string_so_far);
                self.towels.insert(leftover_design.to_string());
                // println!("Yes, {} is possible!", remaining_design);
                return true;
            }

            // Check if there are actually entries in the towels which COULD contain our check string
            // if self
            //     .towels
            //     .iter()
            //     .filter(|s| s.starts_with(check_str))
            //     .count()
            //     == 0
            // {
            //     // println!("No remaining entries in cache that are possible!");
            //     return false;
            // }
        }
        false
    }

    fn is_possible_alt(&mut self, design: &str, already_checked: &String) -> bool {
        if self.towels.contains(design) {
            return true;
        }

        let matching_towels: Vec<_> = self
            .towels
            .iter()
            .filter(|t| design.starts_with(*t))
            .map(|d| d.to_string())
            .collect();

        // println!("Design {} starts with {:?}", design, matching_towels);
        matching_towels.iter().any(|t| {
            // Cache this possible towel arrangement
            let total_so_far = &(already_checked.to_owned() + t);
            self.insert_cache(&total_so_far);

            // let remaining = &design[t.len()..];
            if self.is_possible_alt(&design[t.len()..], total_so_far) {
                self.insert_cache(design);
                return true;
            }
            false
        })

        // for design in &self.designs {
        //     if design.starts_with(design)
        // }
    }

    fn is_possible_alt2(&mut self, design: &str) -> bool {
        let mut stack: Vec<char> = design.chars().collect();
        let mut check_str = "".to_string();

        if self.towels.contains(design) {
            return true;
        }

        // This is not exhaustive - it prioritises smaller segments
        // TODO Need some backtracking to check alternative possibilities
        while let Some(c) = stack.pop() {
            check_str = c.to_string() + &check_str;
            println!("Check string is now {}", check_str);
            if self.towels.contains(&check_str) {
                check_str.clear();
            }
            if check_str.len() > self.max_towel_length {
                stack.extend(check_str.chars().take(check_str.len() - 1));
                check_str.clear();
                continue;
            }
        }

        if check_str.len() > 0 {
            println!("{}", check_str);
            return self.towels.contains(&check_str);
        }
        true
    }

    fn is_possible_alt3(&self, design: &str) -> bool {
        let v: Vec<String> = self.towels.iter().map(|s| s.to_string()).collect();
        let inner: String = v.join("|");
        let pattern = format!(r"^({})+$", inner);
        let re = Regex::new(&pattern).unwrap();
        re.is_match(design)
    }

    fn count_possibilities(&self, design: &str) -> usize {
        let v: Vec<String> = self.towels.iter().map(|s| s.to_string()).collect();
        let inner: String = v.join("|");
        let pattern = format!(r"^({})+$", inner);
        println!("Patt: {}", pattern);
        let re = Regex::new(&pattern).unwrap();
        println!("Design {}.", design);
        for m in re.find_iter(design) {
            println!("{:?}", m);
        }
        0
    }

    fn count_possibilities_alt(&self, design: &str) -> usize {
        let mut counts = HashMap::new();
        counts.insert(0, 1);
        for idx in 1..=design.len() {
            let lower = 0.max(idx.saturating_sub(self.max_towel_length));
            for inner in lower..=idx {
                if self.towels.contains(&design[inner..idx]) {
                    *counts.entry(idx).or_insert(0) += *counts.get(&inner).unwrap_or(&0);
                }
            }
        }

        *counts.get(&(design.len())).unwrap_or(&0)
    }

    fn count_possibilities_alt2(&mut self, design: &str) -> usize {
        if design.is_empty() {
            return 1;
        }

        // Return entry from cache if already existing
        if let Some(count) = self.towel_counts.get(design) {
            return *count;
        }

        // Iterate over towels. Find all the ones that the design starts with.
        let mut result = 0;
        let towels = self.towels.clone();
        for towel in &towels {
            if !design.starts_with(towel) {
                continue;
            }
            result += self.count_possibilities_alt2(&design[towel.len()..])
        }

        self.towel_counts.insert(design.to_owned(), result);
        result
    }

    fn part_a(&mut self) -> usize {
        let designs = self.designs.clone();
        designs.iter().filter(|d| self.is_possible_alt3(d)).count()
    }

    fn part_b(&mut self) -> usize {
        let designs: Vec<_> = self.designs.iter().map(|s| s.to_owned()).collect();
        designs.into_iter().map(|d| self.count_possibilities_alt2(&d)).sum()
        // self.designs
        //     .iter()
        //     .map(|d| self.count_possibilities_alt(d))
        //     .sum()
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let mut towels = TowelDesigns::from_str(puzzle).expect("Could not parse towel designs");
    // 400 is too high
    println!("Part A: {}", towels.part_a());
    println!("Part B: {}", towels.part_b());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_txt_part_a() {
        let puzzle = include_str!("../puzzle/test.txt");
        let mut towels = TowelDesigns::from_str(puzzle).unwrap();
        println!("{:?}", towels);
        assert_eq!(towels.part_a(), 6);
    }

    #[test]
    fn test_test_txt_part_b() {
        let puzzle = include_str!("../puzzle/test.txt");
        let mut towels = TowelDesigns::from_str(puzzle).unwrap();
        println!("{:?}", towels);
        assert_eq!(towels.part_b(), 16);
    }
}

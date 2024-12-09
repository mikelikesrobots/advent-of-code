use std::{collections::HashSet, num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct DiskDefrag {
    pairs: Vec<(i32, i32)>,
}

impl FromStr for DiskDefrag {
    type Err = ParseIntError;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let mut pairs = vec![];
        let trimmed = puzzle.trim().chars().collect::<Vec<char>>();
        for idx in (0..trimmed.len()).step_by(2) {
            let occupied = &trimmed[idx].to_string().parse()?;
            let free = &trimmed.get(idx + 1).unwrap_or(&'0').to_string().parse()?;
            pairs.push((*occupied, *free));
        }
        Ok(DiskDefrag { pairs })
    }
}

impl DiskDefrag {
    fn part_a(&self) -> usize {
        let mut pairs = self.pairs.clone();
        let mut unaccounted: i32 = pairs.iter().map(|(x, _)| x).sum();

        let mut acc = 0;

        let mut start_idx = 0;
        let mut end_idx = pairs.len() - 1;
        let mut memory_idx = 0;

        'outer: while unaccounted > 0 {
            // Take from front for start spaces occupied
            let (start_occ, start_free) = pairs[start_idx];
            for _ in 0..start_occ {
                acc += start_idx * memory_idx;
                memory_idx += 1;
                unaccounted -= 1;
                if unaccounted <= 0 {
                    break 'outer;
                }
            }
            start_idx += 1;

            // Take from end for start spaces unoccupied
            let mut start_free = start_free;
            while start_free > 0 {
                // DANGER - unwrap is okay because end_idx starts within range and goes down
                let (end_occ, _) = pairs.get_mut(end_idx).expect("end_idx not within bounds!");
                if *end_occ > 0 {
                    acc += end_idx * memory_idx;
                    memory_idx += 1;
                    *end_occ -= 1;
                   start_free -= 1;
                   unaccounted -= 1;
                   if unaccounted <= 0 {
                        break 'outer;
                    }
                } else {
                    end_idx -= 1;
                }
            }
        }

        acc
    }

    fn part_b(&self) -> usize {
        let mut unaccounted: i32 = self.pairs.iter().map(|(x, _)| x).sum();
        let mut acc = 0;
        let mut start_idx = 0;
        let end_idx = self.pairs.len() - 1;
        let mut memory_idx: usize = 0;

        let mut copied_memory = HashSet::new();

        'outer: while unaccounted > 0 {

            let (start_occ, start_free) = self.pairs.get(start_idx).expect("Start idx not found!");

            if copied_memory.contains(&start_idx) {
                let (occ, _) = self.pairs.get(start_idx).expect("Start index out of bounds");
                memory_idx += *occ as usize;
            } else {
                // Take from front for start spaces occupied
                for _ in 0..*start_occ {
                    acc += start_idx * memory_idx;
                    memory_idx += 1;
                    unaccounted -= 1;
                    if unaccounted <= 0 {
                        break 'outer;
                    }
                }
            }

            start_idx += 1;
            let mut start_free = *start_free;
            while start_free > 0 {
                let mut memory_copied = false;
                for idx in (start_idx..=end_idx).rev() {
                    if copied_memory.contains(&idx) {
                        continue;
                    }

                    let (end_occ, _) = self.pairs.get(idx).unwrap();
                    if *end_occ > start_free {
                        continue;
                    }

                    // Copy memory over
                    memory_copied = true;
                    for _ in 0..*end_occ {
                        acc += idx * memory_idx;
                        memory_idx += 1;
                        unaccounted -= 1;
                        if unaccounted <= 0 {
                            break 'outer;
                        }
                    }
                    start_free -= *end_occ;

                    copied_memory.insert(idx);
                }

                if !memory_copied {
                    memory_idx += start_free as usize;
                    break;
                }
            }
        }

        acc
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let defrag = DiskDefrag::from_str(puzzle).expect("Unable to parse input");
    println!("Part A: {}", defrag.part_a());
    println!("Part B: {}", defrag.part_b());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_small_txt() {
        // 022111222
        // 2 + 4 + 3 + 4 + 5 + 12 + 14 + 16
        // 60
        let puzzle = include_str!("../puzzle/test_small.txt");
        let defrag = DiskDefrag::from_str(puzzle).unwrap();
        println!("{:?}", defrag);
        let result = defrag.part_a();
        assert_eq!(60, result);
    }

    #[test]
    fn test_test_txt_part_a() {
        let puzzle = include_str!("../puzzle/test.txt");
        let defrag = DiskDefrag::from_str(puzzle).unwrap();
        let result = defrag.part_a();
        assert_eq!(1928, result);
    }

    #[test]
    fn test_test_small_rev_txt() {
        // 000002111
        // 10 + 6 + 7 + 8
        // 31
        let puzzle = include_str!("../puzzle/test_small_rev.txt");
        let defrag = DiskDefrag::from_str(puzzle).unwrap();
        let result = defrag.part_b();
        assert_eq!(31, result);
    }

    #[test]
    fn test_test_txt_part_b() {
        let puzzle = include_str!("../puzzle/test.txt");
        let defrag = DiskDefrag::from_str(puzzle).unwrap();
        let result = defrag.part_b();
        assert_eq!(2858, result);
    }

    #[test]
    fn test_acc() {
        // 308 is correct; 302 is my result
        let input = "00992111777.44.333....5555.6666.....8888..";
        let mut acc: i32 = 0;
        for (idx, c) in input.chars().enumerate() {
            if c.is_ascii_digit() {
                acc += (idx as i32) * c.to_string().parse::<i32>().unwrap();
                println!("{}", acc);
            }
        }
    }
}

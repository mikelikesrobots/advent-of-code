use std::{collections::HashMap, str::FromStr};

struct PrintQueueChecker {
    rules: HashMap<i32, Vec<i32>>,
    page_numbers: Vec<Vec<i32>>,
}

impl FromStr for PrintQueueChecker {
    type Err = anyhow::Error;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let mut rules = HashMap::new();
        puzzle
            .lines()
            .take_while(|l| !l.is_empty())
            .filter_map(|l| {
                let numbers = l.split("|").collect::<Vec<&str>>();
                let left = numbers.first()?.parse::<i32>().ok()?;
                let right = numbers.get(1)?.parse::<i32>().ok()?;
                Some((left, right))
            })
            .for_each(|(left, right)| {
                rules.entry(right).or_insert(vec![]).push(left);
            });

        let page_numbers = puzzle
            .lines()
            .skip_while(|x| !x.is_empty())
            .skip(1)
            .filter_map(|l| l.split(",").map(|x| x.parse().ok()).collect())
            .collect();

        Ok(PrintQueueChecker {
            rules,
            page_numbers,
        })
    }
}

impl PrintQueueChecker {
    fn check_valid(&self, order: &[i32]) -> bool {
        for (idx, val) in order.iter().enumerate() {
            let rule = match self.rules.get(val) {
                Some(rule) => rule,
                None => continue,
            };

            for rule_val in rule {
                if order[idx..].contains(rule_val) {
                    return false;
                }
            }
        }
        true
    }

    fn part_a(&self) -> i32 {
        self.page_numbers
            .iter()
            .filter(|order| self.check_valid(order))
            .map(|order| order[order.len() / 2])
            .sum()
    }

    fn part_b(&self) -> i32 {
        self.page_numbers.iter()
            .filter(|order| !self.check_valid(order))
            .map(|order| {
                let mut order_clone = order.to_vec();
                while !self.check_valid(&order_clone) {
                    let mut found_rule_break = false;
                    let mut old_idx = 0;
                    let mut new_idx = 0;
                    let mut replace_val = 0;

                    for (idx, val) in order_clone.iter().enumerate() {
                        let rule = match self.rules.get(val) {
                            Some(rule) => rule,
                            None => continue,
                        };

                        for (check_idx, check_val) in order_clone.iter().enumerate().skip(idx).rev() {
                            if rule.contains(check_val) {
                                new_idx = check_idx;
                                found_rule_break = true;
                                old_idx = idx;
                                replace_val = *val;
                                break;
                            }
                        };
                    };

                    if found_rule_break {
                        order_clone.remove(old_idx);
                        order_clone.insert(new_idx, replace_val);
                    }
                }
                order_clone[order_clone.len() / 2]
            })
            .sum()
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let checker = PrintQueueChecker::from_str(puzzle).expect("Unable to parse checker");
    println!("Part A: {}", checker.part_a());
    // 6017 is too high
    println!("Part B: {}", checker.part_b());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_parses_correctly() {
        let input = "47|48\n\n47,48,49";
        let checker = PrintQueueChecker::from_str(input).unwrap();

        let expected_rules = {
            let mut rules = HashMap::new();
            rules.insert(48, vec![47]);
            rules
        };

        assert_eq!(checker.rules, expected_rules);
        assert_eq!(checker.page_numbers, vec![vec![47, 48, 49]]);
    }

    #[test]
    fn test_valid_row_part_a() {
        let input = "47|48\n\n47,48,49";
        let checker = PrintQueueChecker::from_str(input).unwrap();
        assert_eq!(checker.part_a(), 48);
    }

    #[test]
    fn test_invalid_row_part_a() {
        let input = "47|48\n\n48,47,49";
        let checker = PrintQueueChecker::from_str(input).unwrap();
        assert_eq!(checker.part_a(), 0);
    }

    #[test]
    fn test_valid_row_part_b() {
        let input = "47|48\n\n47,48,49";
        let checker = PrintQueueChecker::from_str(input).unwrap();
        assert_eq!(checker.part_b(), 0);
    }

    #[test]
    fn test_invalid_row_part_b() {
        let input = "47|48\n48|49\n\n48,47,49";
        let checker = PrintQueueChecker::from_str(input).unwrap();
        assert_eq!(checker.part_b(), 48);
    }

    #[test]
    fn test_another_invalid_row_part_b() {
        let input = "61|13\n29|13\n61|29\n97|61\n\n61,13,29";  // 61, 29, 13
        let checker = PrintQueueChecker::from_str(input).unwrap();
        assert_eq!(checker.part_b(), 29);
    }

    #[test]
    fn test_test_txt_part_a() {
        let input = include_str!("../puzzle/test.txt");
        let checker = PrintQueueChecker::from_str(input).unwrap();
        assert_eq!(checker.part_a(), 143);
    }

    #[test]
    fn test_test_txt_part_b() {
        let input = include_str!("../puzzle/test.txt");
        let checker = PrintQueueChecker::from_str(input).unwrap();
        assert_eq!(checker.part_b(), 123);
    }
}

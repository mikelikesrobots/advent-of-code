use std::{cmp::Ordering, str::FromStr};

#[derive(Debug)]
struct Button {
    x: i64,
    y: i64,
    cost: i64,
}

#[derive(Debug)]
struct ArcadeMachine {
    a: Button,
    b: Button,
    target_x: i64,
    target_y: i64,
}
impl ArcadeMachine {
    fn new_from_strs(a_x: &str, a_y: &str, b_x: &str, b_y: &str, target_x: &str, target_y: &str) -> Result<ArcadeMachine, anyhow::Error> {
        let a_x = a_x.parse()?;
        let a_y = a_y.parse()?;
        let b_x = b_x.parse()?;
        let b_y = b_y.parse()?;
        let target_x = target_x.parse()?;
        let target_y = target_y.parse()?;
        let a = Button { x: a_x, y: a_y, cost: 3};
        let b  = Button { x: b_x, y: b_y, cost: 1};
        Ok(ArcadeMachine { a, b, target_x, target_y })
    }

    fn order_x(&self) -> (&Button, &Button) {
        match self.a.x.cmp(&self.b.x) {
            Ordering::Greater | Ordering::Equal => (&self.b, &self.a),
            Ordering::Less => (&self.a, &self.b),
        }
    }

    fn min_tokens(&self) -> Option<usize> {
        let (min_by_x, max_by_x) = self.order_x();
        let min_iters_x = self.target_x / max_by_x.x;
        let min_iters_y = self.target_y / self.a.y.max(self.b.y);
        let min_iters = min_iters_x.max(min_iters_y);

        for total_button_presses in min_iters..=200 {
            for n in 0..total_button_presses {
                let achieved_x = n * min_by_x.x + (total_button_presses - n) * max_by_x.x;
                if achieved_x != self.target_x {
                    continue;
                }
                let achieved_y = n * min_by_x.y + (total_button_presses - n) * max_by_x.y;
                if achieved_y != self.target_y {
                    continue;
                }
                let min_by_x_tokens = (n * min_by_x.cost) as usize;
                let max_by_x_tokens = ((total_button_presses - n) * max_by_x.cost) as usize;
                return Some(min_by_x_tokens + max_by_x_tokens);
            }
        }

        None
    }

    fn min_tokens_corrected_targets(&self) -> Option<usize> {
        let (min_by_x, max_by_x) = self.order_x();
        let target_x = self.target_x + 1_000_000_000_000;
        let target_y = self.target_y + 1_000_000_000_000;
        let min_iters_x = target_x / max_by_x.x;
        let min_iters_y = target_y / self.a.y.max(self.b.y);
        let min_iters = min_iters_x.max(min_iters_y);
        
        let max_iters_x = target_x / min_by_x.x;
        let max_iters_y = target_y / self.a.y.min(self.b.y);
        let max_iters = max_iters_x.min(max_iters_y);

        println!("Min iters: {}; max iters: {}", min_iters, max_iters);

        for total_button_presses in min_iters..=max_iters {
            for n in 0..total_button_presses {
                let achieved_x: i64 = n * min_by_x.x + (total_button_presses - n) * max_by_x.x;
                if achieved_x != target_x {
                    continue;
                }
                let achieved_y = n * min_by_x.y + (total_button_presses - n) * max_by_x.y;
                if achieved_y != target_y {
                    continue;
                }
                let min_by_x_tokens = (n * min_by_x.cost) as usize;
                let max_by_x_tokens = ((total_button_presses - n) * max_by_x.cost) as usize;
                return Some(min_by_x_tokens + max_by_x_tokens);
            }
        }

        None
    }
}

#[derive(Debug)]
struct Arcade {
    machines: Vec<ArcadeMachine>,
}

impl FromStr for Arcade {
    type Err = anyhow::Error;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"Button A: X\+(\d*), Y\+(\d*)\s*Button B: X\+(\d*), Y\+(\d*)\s*Prize: X=(\d*), Y=(\d*)")?;
        let machines= re.captures_iter(puzzle)
            .map(|c| c.extract())
            .filter_map(|(_, [a_x, a_y, b_x, b_y, target_x, target_y])| ArcadeMachine::new_from_strs(a_x, a_y, b_x, b_y, target_x, target_y).ok())
            .collect::<Vec<_>>();

        Ok(Arcade { machines })
    }
}

impl Arcade {
    fn part_a(&self) -> usize {
        self.machines.iter().filter_map(|m| {
            let tokens = m.min_tokens();
            if let Some(tokens) = tokens {
                println!("{}, {}, {}, {}, {}, {}, {}", tokens, m.a.x, m.a.y, m.b.x, m.b.y, m.target_x, m.target_y);
            }
            tokens
        }).sum()
    }
    fn part_b(&self) -> usize {
        self.machines.iter().filter_map(|m| m.min_tokens_corrected_targets()).sum()
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let arcade = Arcade::from_str(puzzle).expect("Unable to parse puzzle input");
    println!("Part A: {}", arcade.part_a());
    println!("Part B: {}", arcade.part_b());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_test_txt_part_a() {
        let puzzle = include_str!("../puzzle/test.txt");
        let arcade = Arcade::from_str(puzzle).unwrap();
        println!("{:?}", arcade);
        let result = arcade.part_a();
        assert_eq!(result, 480);
    }

    #[test]
    fn test_test_txt_part_b() {
        let puzzle = include_str!("../puzzle/test.txt");
        let arcade = Arcade::from_str(puzzle).unwrap();
        let result = arcade.part_b();
        assert_eq!(result, 480);
    }

    #[test]
    fn test_min_tokens_single_machine() {
        let machine = ArcadeMachine::new_from_strs("94", "34", "22", "67", "8400", "5400").unwrap();
        assert_eq!(machine.min_tokens().unwrap(), 280);
    }

    #[test]
    fn test_min_tokens_increased_targets_single_machine() {
        let machine = ArcadeMachine::new_from_strs("94", "34", "22", "67", "8400", "5400").unwrap();
        assert_eq!(machine.min_tokens_corrected_targets().unwrap(), 280);
    }
}

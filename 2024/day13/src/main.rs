use std::str::FromStr;

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

    fn min_tokens(&self, target_adjust: i64) -> Option<i64> {
        let target_x = self.target_x + target_adjust;
        let target_y = self.target_y + target_adjust;

        let b_multiplied = self.a.y * target_x - self.a.x * target_y;
        let b_divisor = self.a.y * self.b.x - self.a.x * self.b.y;
        let b = b_multiplied / b_divisor;
        let brem = b_multiplied % b_divisor;
        let a_multiplied = target_x - b * self.b.x;
        let a_divisor = self.a.x;
        let a = a_multiplied / a_divisor;
        let arem = a_multiplied % a_divisor;
        if arem != 0 || brem != 0 {
            return None;
        }

        Some(a * self.a.cost + b * self.b.cost)
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
    fn part_a(&self) -> i64 {
        self.machines.iter().filter_map(|m| m.min_tokens(0)).sum()
    }
    fn part_b(&self) -> i64 {
        self.machines.iter().filter_map(|m| m.min_tokens(10_000_000_000_000)).sum()
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
        assert_eq!(result, 875318608908);
    }

    #[test]
    fn test_min_tokens_single_machine() {
        let machine = ArcadeMachine::new_from_strs("94", "34", "22", "67", "8400", "5400").unwrap();
        assert_eq!(machine.min_tokens(0).unwrap(), i64::from(280u32));
    }

    #[test]
    fn test_min_tokens_increased_targets_single_machine() {
        let machine = ArcadeMachine::new_from_strs("94", "34", "22", "67", "8400", "5400").unwrap();
        assert_eq!(machine.min_tokens(1_000_000_000_000), None);
    }

}

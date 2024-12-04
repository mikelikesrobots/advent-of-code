use std::str::FromStr;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

#[derive(Debug)]
struct TobogganComputer {
    instructions: Vec<Instruction>,
}

impl FromStr for TobogganComputer {
    type Err = anyhow::Error;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"mul\((\d*),(\d*)\)|don't\(\)|do\(\)")?;
        let mut instructions = vec![];

        for capture in re.captures_iter(puzzle) {
            let get_int = |idx: usize| capture.get(idx).map_or("i", |m| m.as_str()).parse();
            let instr_raw = capture.get(0).map_or("", |m| m.as_str());

            if instr_raw.contains("mul") {
                let left = get_int(1)?;
                let right = get_int(2)?;
                instructions.push(Instruction::Mul(left, right));
            } else if instr_raw.contains("don't") {
                instructions.push(Instruction::Dont);
            } else if instr_raw.contains("do") {
                instructions.push(Instruction::Do);
            } else {
                return Err(anyhow::Error::msg(format!(
                    "Unrecognised instruction: {:?}",
                    capture.get(1)
                )))
            }
        }

        Ok(TobogganComputer { instructions })
    }
}

impl TobogganComputer {
    fn execute(&self) -> i32 {
        let mut enabled = true;
        let mut acc = 0;
        for instr in &self.instructions {
            match instr {
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = false,
                Instruction::Mul(x, y) => {
                    if enabled {
                        acc += x * y
                    }
                }
            }
        }
        acc
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let computer =
        TobogganComputer::from_str(puzzle).expect("Unable to parse computer instructions.");
    println!("Part A: {}", computer.execute());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computer_executes_mul_only() {
        let instructions = vec![Instruction::Mul(3, 9), Instruction::Mul(10, 2)];
        let expected = 47;
        let computer = TobogganComputer { instructions };
        assert_eq!(expected, computer.execute());
    }

    #[test]
    fn test_computer_executes_with_do_dont() {
        let instructions = vec![
            Instruction::Mul(2, 3), // 6
            Instruction::Dont,
            Instruction::Mul(4, 5), // 20
            Instruction::Do,
            Instruction::Mul(6, 7), // 42
        ];
        let expected = 48;
        let computer = TobogganComputer { instructions };
        assert_eq!(expected, computer.execute());
    }

    #[test]
    fn test_valid_mul_instruction() {
        let instr = "mul(5,4)";
        let result = TobogganComputer::from_str(instr).unwrap().execute();
        assert_eq!(20, result);
    }

    #[test]
    fn test_valid_do_instruction() {
        let instr = "do()";
        let result = TobogganComputer::from_str(instr).unwrap();
        assert_eq!(result.instructions, vec![Instruction::Do]);
    }

    #[test]
    fn test_invalid_mul_instruction() {
        let instr = "fjjjsmul(5,4)_}mul(5.4)";
        let result = TobogganComputer::from_str(instr).unwrap().execute();
        assert_eq!(20, result);
    }

    #[test]
    fn test_test_txt() {
        let instr = include_str!("../puzzle/test.txt");
        let result = TobogganComputer::from_str(instr).unwrap().execute();
        assert_eq!(161, result);
    }

    #[test]
    fn test_test_do_dont_txt() {
        let instr = include_str!("../puzzle/test_do_dont.txt");
        let result = TobogganComputer::from_str(instr).unwrap();
        println!("{:?}", result.instructions);
        assert_eq!(48, result.execute());
    }
}

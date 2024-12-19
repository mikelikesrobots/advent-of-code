use std::str::FromStr;

#[derive(Debug)]
pub struct Computer {
    a: u32,
    b: u32,
    c: u32,
    program: Vec<u32>,
}

impl Clone for Computer {
    fn clone(&self) -> Computer {
        Computer {
            a: self.a,
            b: self.b,
            c: self.c,
            program: self.program.to_vec()
        }
    }
}

impl FromStr for Computer {
    type Err = anyhow::Error;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = puzzle.lines().collect();
        if lines.len() != 5 {
            return Err(anyhow::Error::msg("Invalid length"));
        }

        let a = lines[0]
            .split_whitespace()
            .map(|x| x.parse::<u32>())
            .find(&Result::is_ok)
            .unwrap_or(Ok(0))?;
        let b = lines[1]
            .split_whitespace()
            .map(|x| x.parse::<u32>())
            .find(&Result::is_ok)
            .unwrap_or(Ok(0))?;
        let c = lines[2]
            .split_whitespace()
            .map(|x| x.parse::<u32>())
            .find(&Result::is_ok)
            .unwrap_or(Ok(0))?;

        let program = lines[4]
            .split_whitespace()
            .nth(1)
            .ok_or(anyhow::Error::msg("No program found"))?
            .split(",")
            .filter_map(|n| n.parse().ok())
            .collect();

        Ok(Computer { a, b, c, program })
    }
}

impl Computer {
    fn get_operand(&self, operand: u32) -> Option<u32> {
        match operand {
            0..=3 => Some(operand),
            4 => Some(self.a),
            5 => Some(self.b),
            6 => Some(self.c),
            // 7 is reserved
            _ => None,
        }
    }

    fn tick(&mut self, pc: usize) -> (usize, Option<u32>) {
        let opcode = self.program[pc];
        match opcode {
            0 => {
                let num = self.a;
                let operand = self.program[pc + 1];
                let denom = 2u32.pow(self.get_operand(operand).unwrap());
                self.a = num / denom;
            }
            1 => {
                self.b ^= self.program[pc + 1];
            }
            2 => {
                let combo = self.get_operand(self.program[pc + 1]).unwrap();
                self.b = combo & 0x7;
            }
            3 => {
                if self.a != 0 {
                    return (self.program[pc + 1] as usize, None);
                }
            }
            4 => {
                self.b ^= self.c;
            }
            5 => {
                let operand = self.get_operand(self.program[pc + 1]).unwrap();
                return (pc + 2, Some(operand & 0x7));
            }
            6 => {
                let num = self.a;
                let operand = self.program[pc + 1];
                let denom = 2u32.pow(self.get_operand(operand).unwrap());
                self.b = num / denom;
            }
            7 => {
                let num = self.a;
                let operand = self.program[pc + 1];
                let denom = 2u32.pow(self.get_operand(operand).unwrap());
                self.c = num / denom;
            }
            _ => (),
        }
        (pc + 2, None)
    }

    fn produces_own_program(&mut self) -> bool {
        let mut outputs: Vec<_> = self.program.iter().copied().rev().collect();
        let mut pc = 0;
        while !outputs.is_empty() {
            let (new_pc, out_opt) = self.tick(pc);
            pc = new_pc;
            if let Some(out) = out_opt {
                if let Some(required) = outputs.pop() {
                    if out != required {
                        return false;
                    }
                }
            }
            if pc >= self.program.len() {
                return false;
            }
        }
        true
    }

    fn get_outputs(&mut self) -> String {
        let mut pc = 0;
        let mut outputs: Vec<u32> = vec![];
        while pc < self.program.len() {
            let (new_pc, out_opt) = self.tick(pc);
            pc = new_pc;
            if let Some(out) = out_opt {
                outputs.push(out);
            }
        }
        outputs
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn part_a(&mut self) -> String {
        self.get_outputs()
    }

    pub fn part_b(&mut self) -> u32 {
        let orig_bc = (self.b, self.c);
        // self.a = 117440;
        // if self.produces_own_program() {
        //     return 117440;
        // }
        // 0
        (0..).find(|a_cand| {
            println!("Testing a candidate: {}", a_cand);
            self.a = *a_cand;
            (self.b, self.c) = orig_bc;
            self.produces_own_program()
        }).unwrap()

        // let prog_str = self
        //     .program
        //     .iter()
        //     .map(|n| n.to_string())
        //     .collect::<Vec<String>>()
        //     .join(",");

        // let mut a_cand = 0;
        // loop {
        //     let mut comp = self.clone();
        //     comp.a = a_cand;
        //     let result = comp.compute();
        //     if result == prog_str {
        //         break;
        //     }
        //     a_cand += 1;
        // }
        // a_cand
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_test_txt() {
        let expected = "4,6,3,5,6,3,5,2,1,0";
        let puzzle = include_str!("../puzzle/test.txt");
        let mut computer = Computer::from_str(puzzle).unwrap();
        assert_eq!(expected, computer.part_a());
    }
    #[test]
    fn test_test_own_output_txt() {
        let puzzle = include_str!("../puzzle/test_own_output.txt");
        let mut computer = Computer::from_str(puzzle).unwrap();
        assert_eq!(117440, computer.part_b());
    }
}

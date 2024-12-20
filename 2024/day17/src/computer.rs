use std::str::FromStr;

#[derive(Debug)]
pub struct Computer {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
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
            .map(|x| x.parse::<u64>())
            .find(&Result::is_ok)
            .unwrap_or(Ok(0))?;
        let b = lines[1]
            .split_whitespace()
            .map(|x| x.parse::<u64>())
            .find(&Result::is_ok)
            .unwrap_or(Ok(0))?;
        let c = lines[2]
            .split_whitespace()
            .map(|x| x.parse::<u64>())
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
    fn get_operand(&self, operand: u64) -> Option<u64> {
        match operand {
            0..=3 => Some(operand),
            4 => Some(self.a),
            5 => Some(self.b),
            6 => Some(self.c),
            // 7 is reserved
            _ => None,
        }
    }

    fn tick(&mut self, pc: usize) -> (usize, Option<u64>) {
        let opcode = self.program[pc];
        match opcode {
            0 => {
                self.a >>= self.get_operand(self.program[pc + 1]).unwrap();
            }
            1 => {
                self.b ^= self.program[pc + 1];
            }
            2 => {
                let combo = self.get_operand(self.program[pc + 1]).unwrap();
                self.b = combo & 0b111;
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
                self.b = self.a >> self.get_operand(self.program[pc + 1]).unwrap();
            }
            7 => {
                self.c = self.a >> self.get_operand(self.program[pc + 1]).unwrap();
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

    fn solve_test_txt(&self) -> u64 {
        let mut acc = 0;
        for digit in self.program.iter().rev() {
            acc = ((acc & !0b111) | digit) << 3;
        }
        acc
    }

    pub fn solve_input_txt(&self) -> Option<u64> {
        let mut acc_candidates = vec![0];
        for digit in self.program.iter().rev() {
            let mut current_cands = vec![];
            while let Some(acc) = acc_candidates.pop() {
                for b_cand in 0..8 {
                    let mut b = b_cand ^ 1;
                    let c = acc >> b;
                    b ^= c;
                    b ^= 0b100;
                    b &= 0b111;
                    if b == *digit {
                        let next_acc = ((acc & !0b111) | b_cand) << 3;
                        current_cands.push(next_acc)
                    }
                }
            }
            acc_candidates = current_cands.to_vec();
        }

        let final_candidates: Vec<u64> = acc_candidates.iter().map(|x| x >> 3).collect();

        // Go through our acc candidates to find the one that works
        for cand in final_candidates.iter() {
            let mut comp = self.clone();
            comp.a = *cand;
            if comp.produces_own_program() {
                return Some(*cand);
            }
        }
        None
    }

    fn get_outputs(&mut self) -> String {
        let mut pc = 0;
        let mut outputs: Vec<u64> = vec![];
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

    pub fn part_b(&mut self) -> u64 {
        let orig_bc = (self.b, self.c);
        (0..).find(|a_cand| {
            self.a = *a_cand;
            (self.b, self.c) = orig_bc;
            self.produces_own_program()
        }).unwrap()
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
        let computer = Computer::from_str(puzzle).unwrap();
        assert_eq!(117440, computer.solve_test_txt());
    }
}

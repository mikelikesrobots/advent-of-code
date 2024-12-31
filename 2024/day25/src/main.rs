use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
struct CodeChronicle {
    locks: Vec<u64>,
    keys: Vec<u64>,
}

#[derive(Debug, Error)]
enum CodeChronicleParseErr {
    #[error("Invalid char for lock/key `{0}`")]
    InvalidChar(char),
    #[error("Invalid line length for lock/key `{0}`")]
    InvalidLineLength(String),
    #[error("Set is not lock or key: `{0}`")]
    InvalidSet(String),
}

// 111
// 101
// 000

// 000
// 000
// 111

// 111101000
// 000000111
// AND == 0

impl FromStr for CodeChronicle {
    type Err = CodeChronicleParseErr;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {

        let lines = &mut puzzle.lines();

        let keymask = 0b11111;
        let lockmask = (0b11111) << 30;
        let is_key = |entry: &u64| {entry & keymask > 0};
        let is_lock = |entry: &u64| {entry & lockmask > 0};
        
        let mut keys = vec![];
        let mut locks = vec![];

        loop {
            let set: Vec<_> = lines.take_while(|line| !line.is_empty()).collect();
            if set.is_empty() {
                break;
            }

            let entry = set.iter().try_fold(0, |acc, line| {
                if line.len() != 5{
                    return Err(Self::Err::InvalidLineLength(line.to_string()));
                }
                let line_value = line.chars().try_fold(0, |acc, c| {
                    match c {
                        '#' => Ok((acc << 1) + 1),
                        '.' => Ok(acc << 1),
                        _ => Err(Self::Err::InvalidChar(c)),
                    }
                })?;
                Ok((acc << 5) + line_value)
            })?;

            if is_key(&entry) {
                keys.push(entry);
            } else if is_lock(&entry) {
                locks.push(entry);
            } else {
                let err_string = set.iter().cloned().collect::<String>();
                return Err(Self::Err::InvalidSet(err_string));
            }
        }
        
        Ok(Self { locks, keys })
    }
}

impl CodeChronicle {
    fn part_a(&self) -> usize {
        self.locks.iter().map(|lock| {
            self.keys.iter().filter(|key| {
                // Key and lock fit together?
                lock & *key == 0
            }).count()
        }).sum()
    }
}

fn main() -> Result<(), CodeChronicleParseErr> {
    let puzzle = include_str!("../puzzle/input.txt");
    let chronicle = CodeChronicle::from_str(puzzle)?;
    println!("Part A: {}", chronicle.part_a());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_txt() {
        let puzzle = include_str!("../puzzle/test.txt");
        let chronicle = CodeChronicle::from_str(puzzle).unwrap();
        assert_eq!(chronicle.part_a(), 3);
    }
}

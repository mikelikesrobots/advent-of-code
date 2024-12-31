use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
    str::FromStr,
};

#[derive(Debug, Clone)]
enum WireSource {
    And(String, String),
    Or(String, String),
    Xor(String, String),
    Const(u64),
}

struct CrossedWires {
    wires: HashMap<String, WireSource>,
}

#[derive(Debug)]
enum CrossedWiresParseErr {
    NoWiresFound,
    NoGatesFound,
    BadlyFormedWire,
    BadlyFormedGate,
}

impl FromStr for CrossedWires {
    type Err = CrossedWiresParseErr;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let mut wires = HashMap::new();
        let lines = &mut puzzle.lines();
        for line in &mut lines.take_while(|l| !l.is_empty()) {
            let wire = line
                .split(": ")
                .nth(0)
                .ok_or(CrossedWiresParseErr::BadlyFormedWire)?;
            let val = line
                .split(": ")
                .nth(1)
                .ok_or(CrossedWiresParseErr::BadlyFormedWire)?
                .parse::<u64>()
                .map_err(|_| CrossedWiresParseErr::BadlyFormedWire)?;
            wires.insert(wire.to_string(), WireSource::Const(val));
        }

        let const_len = wires.len();
        if const_len == 0 {
            return Err(CrossedWiresParseErr::NoWiresFound);
        }

        for line in lines {
            let mut parts = line.split_ascii_whitespace();
            let source0 = parts
                .next()
                .ok_or(CrossedWiresParseErr::BadlyFormedGate)?
                .to_string();
            let op = parts.next().ok_or(CrossedWiresParseErr::BadlyFormedGate)?;
            let source1 = parts
                .next()
                .ok_or(CrossedWiresParseErr::BadlyFormedGate)?
                .to_string();
            let result = parts
                .nth(1)
                .ok_or(CrossedWiresParseErr::BadlyFormedGate)?
                .to_string();

            let gate = match op {
                "AND" => WireSource::And(source0, source1),
                "XOR" => WireSource::Xor(source0, source1),
                "OR" => WireSource::Or(source0, source1),
                _ => return Err(CrossedWiresParseErr::BadlyFormedGate),
            };
            wires.insert(result, gate);
        }

        if wires.len() == const_len {
            return Err(CrossedWiresParseErr::NoGatesFound);
        }

        Ok(Self { wires })
    }
}

impl<'a> CrossedWires {
    fn value(wires: &HashMap<String, WireSource>, wire: &String) -> Option<u64> {
        match wires.get(wire) {
            None => None,
            Some(WireSource::Const(x)) => Some(*x),
            Some(WireSource::And(l, r)) => CrossedWires::value(wires, l)
                .and_then(|l| CrossedWires::value(wires, r).map(|r| l & r)),
            Some(WireSource::Xor(l, r)) => CrossedWires::value(wires, l)
                .and_then(|l| CrossedWires::value(wires, r).map(|r| l ^ r)),
            Some(WireSource::Or(l, r)) => CrossedWires::value(wires, l)
                .and_then(|l| CrossedWires::value(wires, r).map(|r| l | r)),
        }
    }

    fn convert_starting_with(wires: &HashMap<String, WireSource>, start: char) -> Option<u64> {
        wires
            .keys()
            .filter(|key| key.starts_with(start))
            .sorted()
            .rev()
            .try_fold(0, |acc, gate| {
                CrossedWires::value(wires, gate).map(|val| (acc << 1) | val)
            })
    }

    fn part_a(&self) -> Option<u64> {
        CrossedWires::convert_starting_with(&self.wires, 'z')
    }

    fn write_wire_dot_file(wires: &HashMap<String, WireSource>, filename: &str) -> Result<(), std::io::Error> {
        let f = File::create(filename)?;
        let mut out = BufWriter::new(f);
        writeln!(out, "digraph {{")?;
        writeln!(out, "  rankdir=\"LR\";")?;
        writeln!(out, "  node [style=filled];")?;

        for (wire, source) in wires.iter() {
            match source {
                // Ignore consts
                WireSource::Const(_) => (),
                WireSource::And(left, right) => {
                    writeln!(out, "  {wire} -> {left};")?;
                    writeln!(out, "  {wire} -> {right};")?;
                    writeln!(out, "  {wire} [label=\"{wire} (AND)\"];")?;
                    writeln!(out, "  {wire} [color=\"red\"];")?;
                }
                WireSource::Or(left, right) => {
                    writeln!(out, "  {wire} -> {left};")?;
                    writeln!(out, "  {wire} -> {right};")?;
                    writeln!(out, "  {wire} [label=\"{wire} (OR)\"];")?;
                    writeln!(out, "  {wire} [color=\"blue\"];")?;
                }
                WireSource::Xor(left, right) => {
                    writeln!(out, "  {wire} -> {left};")?;
                    writeln!(out, "  {wire} -> {right};")?;
                    writeln!(out, "  {wire} [label=\"{wire} (XOR)\"];")?;
                    writeln!(out, "  {wire} [color=\"green\"];")?;
                }
            }
        }
        writeln!(out, "}}")?;

        Ok(())
    }

    // https://www.reddit.com/r/adventofcode/comments/1hl698z/comment/m3v5dfv
    fn visualize(&self) -> Result<String, std::io::Error> {
        CrossedWires::write_wire_dot_file(&self.wires, "day24.dot")?;

        // Swap our suspect wires and write again
        let mut swapped_wires = vec![];
        let mut swap = |left: &str, right: &str, wires: &mut HashMap<String, WireSource>| {
            let (left_val, right_val) = match (wires.get(left), wires.get(right)) {
                (Some(left_val), Some(right_val)) => (left_val.clone(), right_val.clone()),
                _ => return (),
            };
            wires.insert(left.to_string(), right_val);
            wires.insert(right.to_string(), left_val);
            swapped_wires.push(left.to_string());
            swapped_wires.push(right.to_string());
        };
        let mut wires: HashMap<String, WireSource> = self.wires.clone();
        swap("ggk", "rhv", &mut wires);
        swap("z20", "hhh", &mut wires);
        swap("z15", "htp", &mut wires);
        swap("z05", "dkr", &mut wires);

        CrossedWires::write_wire_dot_file(&wires, "day24_swapped.dot")?;

        Ok(swapped_wires.iter().sorted().join(","))
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let wires = CrossedWires::from_str(puzzle).expect("Error parsing puzzle");
    println!("Part A: {:?}", wires.part_a());
    println!("Part B: {:?}", wires.visualize());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_small_txt() {
        let puzzle = include_str!("../puzzle/test_small.txt");
        let wires = CrossedWires::from_str(puzzle).unwrap();
        assert_eq!(Some(4), wires.part_a());
    }

    #[test]
    fn test_test_large_txt() {
        let puzzle = include_str!("../puzzle/test_large.txt");
        let wires = CrossedWires::from_str(puzzle).unwrap();
        assert_eq!(Some(2024), wires.part_a());
    }
}

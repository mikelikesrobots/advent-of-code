use std::str::FromStr;

use computer::Computer;

mod computer;

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let mut computer = Computer::from_str(&puzzle).expect("Could not read program");
    println!("Part A: {}", computer.part_a());
}

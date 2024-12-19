use std::str::FromStr;

use computer::Computer;

mod computer;

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let mut computer = Computer::from_str(puzzle).expect("Could not read program");
    // computer.a = 117440;
    println!("Part A: {}", computer.clone().part_a());
    println!("Part B: {}", computer.part_b());
}
use std::str::FromStr;

use computer::Computer;

mod computer;

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let computer = Computer::from_str(puzzle).expect("Could not read program");
    println!("Part A: {}", computer.clone().part_a());
    println!("Part B: {:?}", computer.solve_input_txt());
}

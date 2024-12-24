use keypad_solver::KeypadSolver;

mod keypad_solver;
mod robot;
mod robot_parse_err;

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let mut simple_solver = KeypadSolver::new_part_a().expect("Could not create simple key solver");
    println!("Part A: {}", simple_solver.sum_keytaps_alt(puzzle));
    let mut complex_solver = KeypadSolver::new_part_b().expect("Could not create complex key solver");
    println!("Part B: {}", complex_solver.sum_keytaps_alt(puzzle));
}

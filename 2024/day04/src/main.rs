
mod direction;
mod point;
mod wordsearch_grid;

use std::str::FromStr;
use wordsearch_grid::WordsearchGrid;

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let grid = WordsearchGrid::from_str(puzzle).unwrap();
    println!("Part A: {}", grid.part_a());
    println!("Part B: {}", grid.part_b());
}

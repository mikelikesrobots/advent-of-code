use std::str::FromStr;

mod maze_cell;
mod reindeer_graph;
mod reindeer_junction_maze;
mod reindeer_maze;
mod reindeer_maze_err;

use reindeer_graph::ReindeerGraph;
// use reindeer_junction_maze::ReindeerJunctionMaze;
// use reindeer_maze::ReindeerMaze;

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let maze = ReindeerGraph::from_str(puzzle).expect("Could not parse maze");
    println!("Part A: {:?}", maze.part_a());
    println!("Part B: {:?}", maze.part_b());
}

#[derive(Debug, PartialEq, Eq)]
pub enum MazeCell {
    Empty,
    Wall,
    Start,
    End,
}

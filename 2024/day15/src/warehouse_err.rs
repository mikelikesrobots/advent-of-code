#[derive(Debug)]
pub enum WarehouseErr {
    NoContentsFound,
    NoRobotFound,
    NoProgramFound,
    UnrecognisedCell,
    UnrecognisedDirection,
}

#[derive(Debug)]
pub enum WarehouseMoveErr {
    InvalidPointMoved,
    MoveObstructed,
    OutOfBounds,
    UnexpectedlyEmptyMoveList,
}

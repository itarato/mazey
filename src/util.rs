use crate::Pair;

pub const NORTH: usize = 0;
pub const EAST: usize = 1;
// pub const SOUTH: usize = 2;
// pub const WEST: usize = 3;

pub enum CellReachType {
    ReachableOnly,
    UnreachableOnly,
    Anything,
}

pub const NEIGHBOUR_MAP: [[i32; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];
pub type Coord = Pair<usize>;

#[derive(Debug, Default)]
pub struct Cell {
    // North > east > south > west.
    pub paths: [bool; 4],
}

impl Cell {
    pub fn new_empty() -> Cell {
        Cell { paths: [false; 4] }
    }

    pub fn new_full() -> Cell {
        Cell { paths: [true; 4] }
    }

    pub fn reachable(&self) -> bool {
        !self.paths[0] || !self.paths[1] || !self.paths[2] || !self.paths[3]
    }
}

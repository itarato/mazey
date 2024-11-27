#[derive(Debug, Default)]
pub struct Cell {
    // North > east > south > west.
    pub paths: [bool; 4],
    force_marked_reached: bool,
}

impl Cell {
    pub fn new_empty() -> Cell {
        Cell {
            paths: [false; 4],
            force_marked_reached: false,
        }
    }

    pub fn new_full() -> Cell {
        Cell {
            paths: [true; 4],
            force_marked_reached: false,
        }
    }

    pub fn reachable(&self) -> bool {
        !self.paths[0]
            || !self.paths[1]
            || !self.paths[2]
            || !self.paths[3]
            || self.force_marked_reached
    }

    pub fn mark_reached(&mut self) {
        self.force_marked_reached = true;
    }
}

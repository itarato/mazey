use crate::cell::*;
use crate::pair::*;
use crate::util::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Maze {
    #[allow(unused)]
    pub fn new_empty(width: usize, height: usize) -> Maze {
        let mut cells: Vec<Cell> = vec![];
        cells.resize_with(width * height, Cell::new_empty);

        Maze {
            width,
            height,
            cells,
        }
    }

    pub fn new_full(width: usize, height: usize) -> Maze {
        let mut cells: Vec<Cell> = vec![];
        cells.resize_with(width * height, Cell::new_full);

        Maze {
            width,
            height,
            cells,
        }
    }

    pub fn connect_cells(&mut self, x: usize, y: usize, dir: usize) {
        let i = y * self.width + x;

        self.cells[i].paths[dir] = false;
        let opposite_cell_x: i32 = x as i32 + NEIGHBOUR_MAP[dir][0];
        let opposite_cell_y: i32 = y as i32 + NEIGHBOUR_MAP[dir][1];

        if opposite_cell_x >= 0
            && opposite_cell_y >= 0
            && opposite_cell_x < self.width as i32
            && opposite_cell_y < self.height as i32
        {
            let opposite_i = opposite_cell_y * self.width as i32 + opposite_cell_x;
            self.cells[opposite_i as usize].paths[(dir + 2) % 4] = false;
        }
    }

    pub fn neighbours(
        &self,
        coord: Pair<usize>,
        reachable_type: CellReachType,
    ) -> HashMap<usize, Pair<usize>> {
        let mut neighbour_coords: HashMap<usize, Pair<usize>> = HashMap::new();
        let mut neighbour_dirs: Vec<usize> = vec![];
        for dir in 0..4 {
            let raw_neighbour_coord = Pair::new(
                coord.x as i32 + NEIGHBOUR_MAP[dir][0],
                coord.y as i32 + NEIGHBOUR_MAP[dir][1],
            );
            if raw_neighbour_coord.x < 0
                || raw_neighbour_coord.y < 0
                || raw_neighbour_coord.x >= self.width as i32
                || raw_neighbour_coord.y >= self.height as i32
            {
                continue;
            }

            match reachable_type {
                CellReachType::ReachableOnly => {
                    if !self.cells[raw_neighbour_coord.index(self.width)].reachable() {
                        continue;
                    }
                }
                CellReachType::UnreachableOnly => {
                    if self.cells[raw_neighbour_coord.index(self.width)].reachable() {
                        continue;
                    }
                }
            }

            neighbour_coords.insert(dir, raw_neighbour_coord.to_usize());
            neighbour_dirs.push(dir);
        }

        neighbour_coords
    }
}

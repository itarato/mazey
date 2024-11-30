use core::f32;
use std::collections::HashMap;

use crate::{
    circle_maze_cell::*,
    util::{CellReachType, Coord},
    Pair,
};

const STARTER_CELL_SIDES: usize = 6;

pub struct CircleMaze {
    pub height: usize,
    pub cells: Vec<Vec<CircleMazeCell>>,
}

impl CircleMaze {
    pub fn new(height: usize) -> CircleMaze {
        let mut cells = vec![];

        let level_height = 30.0;
        let mut current_cell_count = STARTER_CELL_SIDES;
        let cell_arc = 30.0;

        let mut cell_counts = vec![1];

        for h in 1..height {
            let r = (h as f32 - 0.5) * level_height;
            let inner_circumference = 2.0 * r * f32::consts::PI;
            let possible_cell_count = (inner_circumference / cell_arc) as usize;

            if possible_cell_count >= current_cell_count * 2 {
                current_cell_count *= 2;
            }

            cell_counts.push(current_cell_count);
        }

        for i in 0..height {
            let cell_north_path_count = if i == 0 {
                STARTER_CELL_SIDES
            } else if i == height - 1 {
                1
            } else {
                if cell_counts[i + 1] == cell_counts[i] * 2 {
                    2
                } else {
                    1
                }
            };

            let cell_has_default_paths = i != 0;

            cells.push(vec![
                CircleMazeCell::new(
                    cell_has_default_paths,
                    cell_north_path_count
                );
                cell_counts[i]
            ]);
        }

        CircleMaze { height, cells }
    }

    pub fn cell_at(&self, coord: Pair<usize>) -> &CircleMazeCell {
        self.cells
            .get(coord.y)
            .expect("Missing cell row")
            .get(coord.x)
            .expect("Missing cell")
    }

    pub fn cell_at_mut(&mut self, coord: Pair<usize>) -> &mut CircleMazeCell {
        self.cells
            .get_mut(coord.y)
            .expect("Missing cell row")
            .get_mut(coord.x)
            .expect("Missing cell")
    }

    pub fn connect_cells(&mut self, coord: Coord, dir: CircleMazeCellDirection) {
        self.cell_at_mut(coord).open(dir.clone());

        let current_row_len = self.cells[coord.y].len();

        let opposite_dir = match dir {
            CircleMazeCellDirection::East => CircleMazeCellDirection::West,
            CircleMazeCellDirection::West => CircleMazeCellDirection::East,
            CircleMazeCellDirection::North(_) => CircleMazeCellDirection::South,
            CircleMazeCellDirection::South => {
                let cell_row_scale_diff = current_row_len / self.cells[coord.y - 1].len();
                let north_index = coord.x % cell_row_scale_diff;

                CircleMazeCellDirection::North(north_index)
            }
        };

        let opposite_cell = match dir {
            CircleMazeCellDirection::East => self
                .cells
                .get_mut(coord.y)
                .expect("Missing cell row")
                .get_mut((coord.x + 1) % current_row_len)
                .expect("Missing cell"),
            CircleMazeCellDirection::West => self
                .cells
                .get_mut(coord.y)
                .expect("Missing cell row")
                .get_mut((coord.x + current_row_len - 1) % current_row_len)
                .expect("Missing cell"),
            CircleMazeCellDirection::South => {
                let cell_row_scale_diff = current_row_len / self.cells[coord.y - 1].len();
                self.cells
                    .get_mut(coord.y - 1)
                    .expect("Missing cell row")
                    .get_mut(coord.x / cell_row_scale_diff)
                    .expect("Missing cell")
            }
            CircleMazeCellDirection::North(n) => {
                let cell_row_scale_diff = self.cells[coord.y + 1].len() / current_row_len;
                self.cells
                    .get_mut(coord.y + 1)
                    .expect("Missing cell row")
                    .get_mut(coord.x * cell_row_scale_diff + n)
                    .expect("Missing cell")
            }
        };

        opposite_cell.open(opposite_dir);
    }

    pub fn neighbours(
        &self,
        coord: Coord,
        reach_type: CellReachType,
    ) -> HashMap<CircleMazeCellDirection, Coord> {
        let mut unfiltered_neighbours = HashMap::new();
        let current_row_len = self.cells[coord.y].len();

        let current_cell = self.cell_at(coord);

        if current_cell.has_default_paths {
            unfiltered_neighbours.insert(
                CircleMazeCellDirection::East,
                Pair::new((coord.x + 1) % current_row_len, coord.y),
            );
            unfiltered_neighbours.insert(
                CircleMazeCellDirection::West,
                Pair::new((coord.x + current_row_len - 1) % current_row_len, coord.y),
            );

            let lower_row_len = self.cells[coord.y - 1].len();
            let cell_row_scale_diff = current_row_len / lower_row_len;

            unfiltered_neighbours.insert(
                CircleMazeCellDirection::South,
                Pair::new(coord.x / cell_row_scale_diff, coord.y - 1),
            );
        }

        if coord.y < self.height - 1 {
            for n in 0..current_cell.north_path_count {
                let upper_row_len = self.cells[coord.y + 1].len();
                let cell_row_scale_diff = upper_row_len / current_row_len;

                unfiltered_neighbours.insert(
                    CircleMazeCellDirection::North(n),
                    Pair::new(coord.x * cell_row_scale_diff + n, coord.y + 1),
                );
            }
        }

        unfiltered_neighbours
            .into_iter()
            .filter(|(dir, coord)| match reach_type {
                CellReachType::Anything => true,
                CellReachType::ReachableOnly => self.cell_at(*coord).reachable(),
                CellReachType::UnreachableOnly => !self.cell_at(*coord).reachable(),
            })
            .collect()
    }
}

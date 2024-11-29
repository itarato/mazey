use core::f32;

use crate::cell::*;

pub struct CircleMaze {
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl CircleMaze {
    pub fn new(height: usize) -> CircleMaze {
        let mut cells = vec![vec![Cell::new_full()]];

        let level_height = 30.0;
        let mut current_cell_count = 6usize;
        let cell_arc = 15.0;

        for h in 1..height {
            let r = (h as f32 - 0.5) * level_height;
            let inner_circumference = 2.0 * r * f32::consts::PI;
            let possible_cell_count = (inner_circumference / cell_arc) as usize;

            if possible_cell_count >= current_cell_count * 2 {
                current_cell_count *= 2;
            }

            cells.push(vec![Cell::new_full(); current_cell_count]);
        }

        CircleMaze { height, cells }
    }
}

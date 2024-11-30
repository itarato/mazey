use core::f32;

use crate::circle_maze_cell::*;

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
            let cell_path_count = if i == 0 {
                STARTER_CELL_SIDES
            } else if i == height - 1 {
                4
            } else {
                if cell_counts[i + 1] == cell_counts[i] * 2 {
                    5
                } else {
                    4
                }
            };
            cells.push(vec![CircleMazeCell::new(cell_path_count); cell_counts[i]]);
        }

        CircleMaze { height, cells }
    }
}

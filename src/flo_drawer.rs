use flo_draw::{
    canvas::{Color, GraphicsContext, GraphicsPrimitives, LineCap},
    create_drawing_window, with_2d_graphics,
};

use crate::{Maze, Pair};

pub struct FloDrawer;

const CELL_SIZE: f32 = 16.0;
const LINE_WIDTH: f32 = 4.0;
const MAZE_PADDING: f32 = 8.0;
const LINE_MAP: [[f32; 4]; 4] = [
    [0f32, 0f32, 1f32, 0f32],
    [1f32, 0f32, 1f32, 1f32],
    [0f32, 1f32, 1f32, 1f32],
    [0f32, 0f32, 0f32, 1f32],
];

impl FloDrawer {
    pub fn new() -> FloDrawer {
        FloDrawer
    }

    pub fn draw(&self, maze: Maze, solution: Vec<Pair<usize>>) {
        with_2d_graphics(move || {
            let canvas = create_drawing_window("Mazey");

            canvas.draw(|gc| {
                let w: f32 = CELL_SIZE * maze.width as f32;
                let h: f32 = CELL_SIZE * maze.height as f32;

                gc.clear_canvas(Color::Rgba(0.1, 0.1, 0.1, 1.0));
                gc.canvas_height(h + (MAZE_PADDING * 2.0));
                gc.center_region(MAZE_PADDING, -MAZE_PADDING, w, h + MAZE_PADDING);

                gc.stroke_color(Color::Rgba(0.2, 0.5, 0.8, 1.0));
                gc.line_width(LINE_WIDTH);
                gc.line_cap(LineCap::Round);

                gc.rect(0.0, 0.0, w as f32, h as f32);
                gc.stroke();

                for y in 0..maze.height {
                    for x in 0..maze.width {
                        let i = y * maze.width + x;

                        let start_x: f32 = x as f32 * CELL_SIZE as f32;
                        let start_y: f32 = y as f32 * CELL_SIZE as f32;

                        for dir in 0..2 {
                            if !maze.cells[i].paths[dir] {
                                continue;
                            }

                            gc.new_path();
                            gc.move_to(
                                start_x + (CELL_SIZE * LINE_MAP[dir][0]),
                                start_y + (CELL_SIZE * LINE_MAP[dir][1]),
                            );
                            gc.line_to(
                                start_x + (CELL_SIZE * LINE_MAP[dir][2]),
                                start_y + (CELL_SIZE * LINE_MAP[dir][3]),
                            );
                            gc.stroke();
                        }
                    }
                }

                if !solution.is_empty() {
                    gc.stroke_color(Color::Rgba(0.9, 0.4, 0.6, 1.0));
                    for i in 0..solution.len() - 1 {
                        gc.move_to(
                            (solution[i].x as f32 + 0.5) * CELL_SIZE,
                            (solution[i].y as f32 + 0.5) * CELL_SIZE,
                        );
                        gc.line_to(
                            (solution[i + 1].x as f32 + 0.5) * CELL_SIZE,
                            (solution[i + 1].y as f32 + 0.5) * CELL_SIZE,
                        );
                        gc.stroke();
                    }
                }
            });
        });
    }
}

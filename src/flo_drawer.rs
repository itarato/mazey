use flo_draw::{
    canvas::{Color, GraphicsContext},
    create_drawing_window, with_2d_graphics,
};

use crate::Maze;

pub struct FloDrawer;

impl FloDrawer {
    pub fn new() -> FloDrawer {
        FloDrawer
    }

    pub fn draw(&self, maze: Maze) {
        with_2d_graphics(|| {
            let canvas = create_drawing_window("Mazey");

            canvas.draw(|gc| {
                gc.clear_canvas(Color::Rgba(0.1, 0.1, 0.1, 1.0));
                gc.canvas_height(1024.0);
                gc.center_region(0.0, 0.0, 1024.0, 1024.0);

                gc.new_path();
                gc.move_to(0.0, 0.0);
                gc.line_to(300.0, 300.0);
                gc.stroke_color(Color::Rgba(0.9, 0.9, 0.9, 1.0));
                gc.stroke();

                dbg!(maze);
            });
        });
    }
}

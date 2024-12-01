use flo_draw::{
    canvas::{Color, GraphicsContext, GraphicsPrimitives, LineCap},
    create_drawing_window, with_2d_graphics,
};

use crate::{circle_maze::CircleMaze, util::Coord, Maze, Pair};

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
    #[allow(unused)]
    pub fn draw(
        maze: Maze,
        solution: Vec<Pair<usize>>,
        max_distance: i32,
        distance_map: Vec<Vec<i32>>,
    ) {
        with_2d_graphics(move || {
            let canvas = create_drawing_window("Mazey");

            canvas.draw(|gc| {
                let w: f32 = CELL_SIZE * maze.width as f32;
                let h: f32 = CELL_SIZE * maze.height as f32;

                gc.clear_canvas(Color::Rgba(0.1, 0.1, 0.1, 1.0));
                gc.canvas_height(h + (MAZE_PADDING * 2.0));
                gc.center_region(0.0, -MAZE_PADDING, w, h + MAZE_PADDING);

                for y in 0..maze.height {
                    for x in 0..maze.width {
                        let distance_intensity =
                            (max_distance - distance_map[y][x]) as f32 / max_distance as f32;
                        gc.fill_color(Color::Rgba(
                            distance_intensity / 5.0,
                            distance_intensity / 1.2,
                            distance_intensity / 5.0,
                            1.0,
                        ));
                        gc.rect(
                            x as f32 * CELL_SIZE,
                            y as f32 * CELL_SIZE,
                            (x + 1) as f32 * CELL_SIZE,
                            (y + 1) as f32 * CELL_SIZE,
                        );
                        gc.fill();
                    }
                }

                gc.stroke_color(Color::Rgba(0.5, 0.6, 0.7, 1.0));
                gc.line_width(LINE_WIDTH);
                gc.line_cap(LineCap::Round);

                // Border
                gc.rect(0.0, 0.0, w, h);
                gc.stroke();

                // Walls.
                for y in 0..maze.height {
                    for x in 0..maze.width {
                        let i = y * maze.width + x;

                        let start_x: f32 = x as f32 * CELL_SIZE;
                        let start_y: f32 = y as f32 * CELL_SIZE;

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

                // Solution.
                if !solution.is_empty() {
                    gc.stroke_color(Color::Rgba(1.0, 0.4, 0.1, 1.0));
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

    pub fn draw_circle_maze(maze: CircleMaze, solution: Vec<Coord>) {
        with_2d_graphics(move || {
            let canvas = create_drawing_window("Mazey");

            let level_height = 40.0f32;
            let w = level_height * (maze.height as f32 * 2.0 - 1.0);
            let h = w;

            let offset_x = w / 2.0;
            let offset_y = h / 2.0;

            canvas.draw(|gc| {
                gc.clear_canvas(Color::Rgba(0.1, 0.1, 0.1, 1.0));
                gc.canvas_height(h + (MAZE_PADDING * 2.0));
                gc.center_region(0.0, -MAZE_PADDING, w, h + MAZE_PADDING);

                gc.stroke_color(Color::Rgba(0.65, 0.7, 0.75, 1.0));
                gc.line_width(LINE_WIDTH);
                gc.line_cap(LineCap::Round);

                for h in 0..maze.height {
                    let r = (h as f32 + 0.5) * level_height;
                    let cell_count = if h == maze.height - 1 {
                        maze.cells[h].len()
                    } else {
                        maze.cells[h + 1].len()
                    };

                    for i in 0..cell_count {
                        // "Top" line start.
                        let alpha_from = (360.0 / cell_count as f32) * i as f32;
                        let alpha_from_rad = (alpha_from / 180.0) * std::f32::consts::PI;
                        let beta_from_rad = ((90.0 - alpha_from) / 180.0) * std::f32::consts::PI;
                        let x_from = r * alpha_from_rad.sin();
                        let y_from = r * beta_from_rad.sin();

                        // "Top" line finish.
                        let alpha_to = (360.0 / cell_count as f32) * (i + 1) as f32;
                        let alpha_to_rad = (alpha_to / 180.0) * std::f32::consts::PI;
                        let beta_to_rad = ((90.0 - alpha_to) / 180.0) * std::f32::consts::PI;
                        let x_to = r * alpha_to_rad.sin();
                        let y_to = r * beta_to_rad.sin();

                        if h == maze.height - 1
                            || !maze
                                .cell_at(Pair::new(i, h + 1))
                                .is_open_at(crate::circle_maze_cell::CircleMazeCellDirection::South)
                        {
                            gc.move_to(x_from + offset_x, y_from + offset_y);
                            gc.line_to(x_to + offset_x, y_to + offset_y);
                            gc.stroke();
                        }

                        // "Side" (left) wall.
                        if h < maze.height - 1
                            && !maze
                                .cell_at(Pair::new(i, h + 1))
                                .is_open_at(crate::circle_maze_cell::CircleMazeCellDirection::West)
                        {
                            // Inner end.
                            let r_inner = (h as f32 + 1.5) * level_height;
                            let x_to = r_inner * alpha_from_rad.sin();
                            let y_to = r_inner * beta_from_rad.sin();

                            // Outer end.
                            gc.move_to(x_from + offset_x, y_from + offset_y);
                            gc.line_to(x_to + offset_x, y_to + offset_y);
                            gc.stroke();
                        }
                    }
                }

                // Solution.
                if solution.len() > 1 {
                    gc.stroke_color(Color::Rgba(1.0, 0.4, 0.1, 1.0));
                    for i in 1..solution.len() {
                        let (x_from, y_from) = FloDrawer::circle_maze_pos_for_cell(
                            &maze,
                            solution[i - 1],
                            level_height,
                        );
                        let (x_to, y_to) =
                            FloDrawer::circle_maze_pos_for_cell(&maze, solution[i], level_height);
                        gc.move_to(x_from + offset_x, y_from + offset_y);
                        gc.line_to(x_to + offset_x, y_to + offset_y);
                        gc.stroke();
                    }
                }
            });
        });
    }

    fn circle_maze_pos_for_cell(
        maze: &CircleMaze,
        cell_coord: Coord,
        row_height: f32,
    ) -> (f32, f32) {
        let row_lenght = maze.cells[cell_coord.y].len();
        let r = row_height * cell_coord.y as f32;
        let alpha = (360.0 / row_lenght as f32) * (cell_coord.x as f32 + 0.5);
        let alpha_rad = (alpha / 180.0) * std::f32::consts::PI;
        let beta_rad = ((90.0 - alpha) / 180.0) * std::f32::consts::PI;
        let x = r * alpha_rad.sin();
        let y = r * beta_rad.sin();

        (x, y)
    }
}

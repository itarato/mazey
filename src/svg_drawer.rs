use draw::{
    render, shape::LinePoint, Canvas, Color, Drawing, Point, Shape, Style, SvgRenderer, RGB,
};

use crate::{Maze, Pair};

pub struct SvgDrawer;

impl SvgDrawer {
    #[allow(unused)]
    pub fn draw(maze: &Maze, cell_size: u32, wall_thickness: u32, solution: Vec<Pair<usize>>) {
        let cell_size_f32 = cell_size as f32;
        let w: u32 = cell_size * maze.width as u32;
        let h: u32 = cell_size * maze.height as u32;
        let mut canvas: Canvas = Canvas::new(w, h);

        canvas.display_list.add(
            Drawing::new()
                .with_shape(Shape::Rectangle {
                    width: w,
                    height: h,
                })
                .with_style(Style::stroked(wall_thickness, Color::black())),
        );

        let line_map = vec![
            vec![0f32, 0f32, 1f32, 0f32],
            vec![1f32, 0f32, 1f32, 1f32],
            vec![0f32, 1f32, 1f32, 1f32],
            vec![0f32, 0f32, 0f32, 1f32],
        ];

        let line_ending_adjustment = vec![
            vec![-1f32, 0f32, 1f32, 0f32],
            vec![0f32, -1f32, 0f32, 1f32],
            vec![-1f32, 0f32, 1f32, 0f32],
            vec![0f32, -1f32, 0f32, 1f32],
        ];

        for y in 0..maze.height {
            for x in 0..maze.width {
                let i = y * maze.width + x;

                let start_x: f32 = x as f32 * cell_size as f32;
                let start_y: f32 = y as f32 * cell_size as f32;

                for dir in 0..2 {
                    if !maze.cells[i].paths[dir] {
                        continue;
                    }

                    canvas.display_list.add(
                        Drawing::new()
                            .with_shape(Shape::Line {
                                start: Point {
                                    x: start_x
                                        + (cell_size_f32 * line_map[dir][0])
                                        + line_ending_adjustment[dir][0],
                                    y: start_y
                                        + (cell_size_f32 * line_map[dir][1])
                                        + line_ending_adjustment[dir][1],
                                },
                                points: vec![LinePoint::Straight {
                                    point: Point {
                                        x: start_x
                                            + (cell_size_f32 * line_map[dir][2])
                                            + line_ending_adjustment[dir][2],
                                        y: start_y
                                            + (cell_size_f32 * line_map[dir][3])
                                            + line_ending_adjustment[dir][3],
                                    },
                                }],
                            })
                            .with_style(Style::stroked(wall_thickness, Color::black())),
                    );
                }
            }
        }

        if !solution.is_empty() {
            for i in 0..solution.len() - 1 {
                canvas.display_list.add(
                    Drawing::new()
                        .with_shape(Shape::Line {
                            start: Point {
                                x: (solution[i].x as f32 + 0.5) * cell_size_f32,
                                y: (solution[i].y as f32 + 0.5) * cell_size_f32,
                            },
                            points: vec![LinePoint::Straight {
                                point: Point {
                                    x: (solution[i + 1].x as f32 + 0.5) * cell_size_f32,
                                    y: (solution[i + 1].y as f32 + 0.5) * cell_size_f32,
                                },
                            }],
                        })
                        .with_style(Style::stroked(wall_thickness, RGB::new(200, 40, 40))),
                );
            }
        }

        render::save(&canvas, "./mazey.svg", SvgRenderer::new()).expect("Image write has failed");
    }
}

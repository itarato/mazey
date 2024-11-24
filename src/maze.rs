use std::collections::HashMap;
use std::collections::HashSet;

use draw::{
    render, shape::LinePoint, Canvas, Color, Drawing, Point, Shape, Style, SvgRenderer, RGB,
};

use crate::cell::*;
use crate::pair::*;
use crate::util::*;

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

    #[allow(unused)]
    pub fn dump_ascii(&self, solution: Vec<Pair<usize>>) {
        let solution_set: HashSet<Pair<usize>> = solution.into_iter().collect();

        for y in 0..self.height {
            print!("█");
            for x in 0..self.width {
                let i = y * self.width + x;

                if self.cells[i].paths[NORTH] {
                    print!("██");
                } else {
                    print!(" █");
                }
            }
            print!("\n");

            print!("█");
            for x in 0..self.width {
                let i = y * self.width + x;

                if solution_set.contains(&Pair::new(x, y)) {
                    print!("x");
                } else {
                    print!(" ");
                }

                if self.cells[i].paths[EAST] {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            print!("\n")
        }

        print!("█");
        for _ in 0..self.width {
            print!("██");
        }

        print!("\n\n");
    }

    #[allow(unused)]
    pub fn dump_image_file(&self, cell_size: u32, wall_thickness: u32, solution: Vec<Pair<usize>>) {
        let cell_size_f32 = cell_size as f32;
        let w: u32 = cell_size * self.width as u32;
        let h: u32 = cell_size * self.height as u32;
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

        for y in 0..self.height {
            for x in 0..self.width {
                let i = y * self.width + x;

                let start_x: f32 = x as f32 * cell_size as f32;
                let start_y: f32 = y as f32 * cell_size as f32;

                for dir in 0..2 {
                    if !self.cells[i].paths[dir] {
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

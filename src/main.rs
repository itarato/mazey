use std::env::args;

use draw::{render, shape::LinePoint, Canvas, Color, Drawing, Point, Shape, Style, SvgRenderer};
use rand::prelude::*;

const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;

const NEIGHBOUR_MAP: [[i32; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];

#[derive(Debug, Default)]
struct Cell {
    // North > east > south > west.
    paths: [bool; 4],
}

impl Cell {
    fn new_empty() -> Cell {
        Cell { paths: [false; 4] }
    }

    fn new_full() -> Cell {
        Cell { paths: [true; 4] }
    }
}

struct Maze {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Maze {
    #[allow(unused)]
    fn new_empty(width: usize, height: usize) -> Maze {
        let mut cells: Vec<Cell> = vec![];
        cells.resize_with(width * height, Cell::new_empty);

        Maze {
            width,
            height,
            cells,
        }
    }

    fn new_full(width: usize, height: usize) -> Maze {
        let mut cells: Vec<Cell> = vec![];
        cells.resize_with(width * height, Cell::new_full);

        Maze {
            width,
            height,
            cells,
        }
    }

    fn connect_cells(&mut self, x: usize, y: usize, dir: usize) {
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

    #[allow(unused)]
    fn binary_tree_maze_creation(&mut self) {
        let mut rng = rand::thread_rng();

        for y in 0..self.height {
            for x in 0..self.width {
                if y == 0 && x == self.width - 1 {
                    continue;
                } else {
                    let dir = if y == 0 {
                        EAST
                    } else if x == self.width - 1 {
                        NORTH
                    } else {
                        rng.gen_range(0..=1)
                    };

                    self.connect_cells(x, y, dir);
                }
            }
        }
    }

    #[allow(unused)]
    fn sidewinder_maze_creation(&mut self) {
        let mut rng = rand::thread_rng();
        let mut run_length: usize;

        for y in 0..self.height {
            run_length = 0;

            for x in 0..self.width {
                let i = y * self.width + x;

                if y == 0 && x == self.width - 1 {
                    continue;
                } else {
                    if x == self.width - 1 {
                        // Check length of run.
                        // Pick on randomly and erast north.
                        let run_rand_i = rng.gen_range(0..=run_length);
                        self.connect_cells(x - run_rand_i, y, NORTH);

                        run_length = 0;
                    } else if y == 0 {
                        self.connect_cells(x, y, EAST);
                    } else {
                        if rng.gen_range(0..=1) == 0 {
                            // Check length of run.
                            // Pick on randomly and erast north.
                            let run_rand_i = rng.gen_range(0..=run_length);
                            self.connect_cells(x - run_rand_i, y, NORTH);

                            run_length = 0;
                        } else {
                            self.connect_cells(x, y, EAST);
                            run_length += 1;
                        }
                    }
                }
            }
        }
    }

    #[allow(unused)]
    fn dump_ascii(&self) {
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

                if self.cells[i].paths[EAST] {
                    print!(" █");
                } else {
                    print!("  ");
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
    fn dump_image_file(&self, cell_size: u32, wall_thickness: u32) {
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

        render::save(&canvas, "./mazey.svg", SvgRenderer::new()).expect("Image write has failed");
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    let width = args
        .get(1)
        .and_then(|raw_width| usize::from_str_radix(&raw_width, 10).ok())
        .unwrap_or(10);
    let height = args
        .get(2)
        .and_then(|raw_width| usize::from_str_radix(&raw_width, 10).ok())
        .unwrap_or(10);

    let mut maze = Maze::new_full(width, height);
    maze.sidewinder_maze_creation();
    // maze.binary_tree_maze_creation();
    // maze.dump_ascii();
    maze.dump_image_file(8, 2);
}

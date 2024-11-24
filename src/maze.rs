use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use draw::{
    render, shape::LinePoint, Canvas, Color, Drawing, Point, Shape, Style, SvgRenderer, RGB,
};
use rand::prelude::*;

use crate::cell::*;
use crate::pair::*;
use crate::util::*;
use rand::seq::SliceRandom;
use std::cmp::min;

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
    pub fn binary_tree_maze_creation(&mut self) {
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
    pub fn sidewinder_maze_creation(&mut self) {
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
    pub fn random_maze_creation(&mut self, start: Pair<usize>) {
        let mut unreachable_cells: HashSet<Pair<usize>> = HashSet::new();
        for y in 0..self.height {
            for x in 0..self.width {
                unreachable_cells.insert(Pair::new(x, y));
            }
        }

        let mut rnd = thread_rng();

        let mut work_queue: VecDeque<Pair<usize>> = VecDeque::new();
        work_queue.push_back(start);
        unreachable_cells.remove(&start);

        loop {
            while let Some(current_coord) = work_queue.pop_back() {
                let neighbour_coords =
                    self.neighbours(current_coord, CellReachType::UnreachableOnly);
                let mut neighbour_dirs = neighbour_coords.keys().collect::<Vec<_>>();

                let used_neighbour_count =
                    rnd.gen_range(min(1, neighbour_coords.len())..=min(2, neighbour_coords.len()));

                neighbour_dirs.shuffle(&mut rnd);
                for i in 0..used_neighbour_count {
                    self.connect_cells(current_coord.x, current_coord.y, *neighbour_dirs[i]);
                    work_queue.push_back(neighbour_coords[&neighbour_dirs[i]]);
                    unreachable_cells.remove(&neighbour_coords[&neighbour_dirs[i]]);
                }
            }

            // Check for unreachable cells.
            if unreachable_cells.is_empty() {
                break;
            }

            for unreachable_cell in unreachable_cells.clone() {
                let neighbour_coords =
                    self.neighbours(unreachable_cell, CellReachType::ReachableOnly);

                if !neighbour_coords.is_empty() {
                    let random_reachable_neighbour_dir = neighbour_coords.keys().next().unwrap();
                    self.connect_cells(
                        unreachable_cell.x,
                        unreachable_cell.y,
                        *random_reachable_neighbour_dir,
                    );
                    unreachable_cells.remove(&unreachable_cell);
                    work_queue.push_back(unreachable_cell);

                    break;
                }
            }

            if work_queue.is_empty() {
                panic!("work queue should have a new item");
            }
        }
    }

    fn neighbours(
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
    pub fn dijkstra_path_finding(
        &self,
        start: Pair<usize>,
        finish: Pair<usize>,
    ) -> Vec<Pair<usize>> {
        let mut distance_map: Vec<Vec<i32>> = vec![vec![-1; self.width]; self.height];

        let mut work_queue: VecDeque<Pair<usize>> = VecDeque::new();
        work_queue.push_back(start);
        distance_map[start.y][start.x] = 0;

        let mut completed = false;

        while let Some(current_coord) = work_queue.pop_front() {
            for dir in 0..4 {
                let neighbour_coord = Pair::new(
                    current_coord.x as i32 + NEIGHBOUR_MAP[dir][0],
                    current_coord.y as i32 + NEIGHBOUR_MAP[dir][1],
                );

                if neighbour_coord.x < 0
                    || neighbour_coord.y < 0
                    || neighbour_coord.x >= self.width as i32
                    || neighbour_coord.y >= self.height as i32
                {
                    continue;
                }

                let current_cell =
                    &self.cells[(current_coord.y * self.width + current_coord.x) as usize];
                if current_cell.paths[dir] {
                    // It's a wall.
                    continue;
                }

                let current_distance = distance_map[current_coord.y][current_coord.x];
                let neighbour_distance =
                    distance_map[neighbour_coord.y as usize][neighbour_coord.x as usize];

                if neighbour_distance != -1 {
                    if neighbour_distance > current_distance + 1 {
                        panic!("This was not suppose to happen with breadth first search.");
                    }
                    // Already visited.
                    continue;
                }

                distance_map[neighbour_coord.y as usize][neighbour_coord.x as usize] =
                    current_distance + 1;
                let neighbour_coord_usize =
                    Pair::new(neighbour_coord.x as usize, neighbour_coord.y as usize);

                if neighbour_coord_usize == finish {
                    completed = true;
                    break;
                }

                work_queue.push_back(neighbour_coord_usize);
            }

            if completed {
                break;
            }
        }

        // dbg!(&distance_map);

        // Extract path.
        let mut current_distance = distance_map[finish.y][finish.x];
        if current_distance == -1 {
            panic!("Haven't found path.");
        }

        let mut path: Vec<Pair<usize>> = vec![];
        let mut current_coord = finish;
        let mut found_next;

        path.push(current_coord);

        loop {
            found_next = false;

            for dir in 0..4 {
                let neighbour_coord = Pair::new(
                    current_coord.x as i32 + NEIGHBOUR_MAP[dir][0],
                    current_coord.y as i32 + NEIGHBOUR_MAP[dir][1],
                );

                if neighbour_coord.x < 0
                    || neighbour_coord.y < 0
                    || neighbour_coord.x >= self.width as i32
                    || neighbour_coord.y >= self.height as i32
                {
                    continue;
                }

                if distance_map[neighbour_coord.y as usize][neighbour_coord.x as usize]
                    == current_distance - 1
                    && !self.cells[current_coord.y * self.width + current_coord.x].paths[dir]
                {
                    current_distance -= 1;
                    current_coord =
                        Pair::new(neighbour_coord.x as usize, neighbour_coord.y as usize);

                    path.push(current_coord);

                    found_next = true;

                    break;
                }
            }

            if current_coord == start {
                break;
            }

            if found_next {
                continue;
            }

            panic!("Missing previous step.");
        }

        path.reverse();

        return path;
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

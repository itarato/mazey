mod ascii_drawer;
mod cell;
mod circle_maze;
mod circle_maze_cell;
mod flo_drawer;
mod maze;
mod maze_builder;
mod pair;
mod solver;
mod svg_drawer;
mod util;

use circle_maze::CircleMaze;
use flo_drawer::*;
use maze::*;
use maze_builder::*;
use pair::*;
use solver::*;
use std::env::args;

fn maze_example() {
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
    let start = Pair::new(0, 0);
    let finish = Pair::new(width - 1, height - 1);

    // MazeBuilder::random_maze_creation(&mut maze, start);
    // MazeBuilder::aldous_broder_maze_creation(&mut maze, start);
    // MazeBuilder::wilson_maze_creation(&mut maze, start);

    let solution = Solver::dijkstra_path_finding_solver(&maze, start, finish);
    let (max_distance, distance_map) = Solver::build_distance_map(&maze, start);

    FloDrawer::draw(maze, solution, max_distance, distance_map);
}

fn main() {
    let mut circle_maze = CircleMaze::new(12);
    // dbg!(circle_maze.neighbours(Pair::new(0, 4)));

    // circle_maze.connect_cells(
    //     Pair::new(1, 1),
    //     circle_maze_cell::CircleMazeCellDirection::South,
    // );
    // circle_maze.connect_cells(
    //     Pair::new(1, 2),
    //     circle_maze_cell::CircleMazeCellDirection::East,
    // );

    // for i in 0..12 {
    //     dbg!(circle_maze.cells[i].len());
    //     // dbg!(circle_maze.cells[i][0].paths.len());
    // }
    FloDrawer::draw_circle_maze(circle_maze);
}

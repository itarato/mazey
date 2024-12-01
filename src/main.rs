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

#[allow(unused)]
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
    let mut circle_maze = CircleMaze::new(32);
    let start = Pair::new(0, 0);
    let finish = Pair::new(0, 31);
    MazeBuilder::random_circle_maze_creation(&mut circle_maze, start);
    let solution =
        Solver::dijkstra_path_finding_solver_for_circle_maze(&circle_maze, start, finish);
    // dbg!(solution);
    FloDrawer::draw_circle_maze(circle_maze, solution);
}

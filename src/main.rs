mod ascii_drawer;
mod cell;
mod flo_drawer;
mod maze;
mod maze_builder;
mod pair;
mod solver;
mod svg_drawer;
mod util;

use flo_drawer::*;
use maze::*;
use maze_builder::*;
use pair::*;
use solver::*;
use std::env::args;

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
    let start = Pair::new(0, 0);
    let finish = Pair::new(width - 1, height - 1);

    MazeBuilder::random_maze_creation(&mut maze, start);
    // MazeBuilder::aldous_broder_maze_creation(&mut maze, start);
    // MazeBuilder::wilson_maze_creation(&mut maze, start);

    let solution = Solver::dijkstra_path_finding_solver(&maze, start, finish);
    let (max_distance, distance_map) = Solver::build_distance_map(&maze, start);

    let flo_drawer_instance = FloDrawer::new();
    flo_drawer_instance.draw(maze, solution, max_distance, distance_map);
}

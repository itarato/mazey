mod ascii_drawer;
mod cell;
mod flo_drawer;
mod maze;
mod maze_builder;
mod pair;
mod solver;
mod svg_drawer;
mod util;

use ascii_drawer::*;
use flo_drawer::*;
use maze::*;
use maze_builder::*;
use pair::*;
use solver::*;
use std::env::args;
use svg_drawer::*;

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

    MazeBuilder::random_maze_creation(&mut maze, Pair::new(0, 0));
    // maze.sidewinder_maze_creation();
    // dbg!(&maze);
    // maze.binary_tree_maze_creation();

    let solution = Solver::dijkstra_path_finding_solver(
        &maze,
        Pair::new(0, 0),
        Pair::new(width - 1, height - 1),
    );
    // dbg!(&solution);

    // maze.dump_ascii(solution);
    // maze.dump_image_file(8, 2, solution);

    let flo_drawer_instance = FloDrawer::new();
    flo_drawer_instance.draw(maze);
}

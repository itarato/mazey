mod cell;
mod maze;
mod pair;
mod util;

use maze::*;
use pair::*;
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

    maze.random_maze_creation(Pair::new(0, 0));
    // maze.sidewinder_maze_creation();
    // dbg!(&maze);
    // maze.binary_tree_maze_creation();

    let solution = maze.dijkstra_path_finding(Pair::new(0, 0), Pair::new(width - 1, height - 1));
    // dbg!(&solution);

    // maze.dump_ascii(solution);
    maze.dump_image_file(8, 2, solution);
}

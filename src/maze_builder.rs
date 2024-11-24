use crate::util::*;
use crate::Maze;
use crate::Pair;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::cmp::min;
use std::collections::HashSet;
use std::collections::VecDeque;

pub struct MazeBuilder;

impl MazeBuilder {
    #[allow(unused)]
    pub fn binary_tree_maze_creation(maze: &mut Maze) {
        let mut rng = rand::thread_rng();

        for y in 0..maze.height {
            for x in 0..maze.width {
                if y == 0 && x == maze.width - 1 {
                    continue;
                } else {
                    let dir = if y == 0 {
                        EAST
                    } else if x == maze.width - 1 {
                        NORTH
                    } else {
                        rng.gen_range(0..=1)
                    };

                    maze.connect_cells(x, y, dir);
                }
            }
        }
    }

    #[allow(unused)]
    pub fn sidewinder_maze_creation(maze: &mut Maze) {
        let mut rng = rand::thread_rng();
        let mut run_length: usize;

        for y in 0..maze.height {
            run_length = 0;

            for x in 0..maze.width {
                let i = y * maze.width + x;

                if y == 0 && x == maze.width - 1 {
                    continue;
                } else {
                    if x == maze.width - 1 {
                        // Check length of run.
                        // Pick on randomly and erast north.
                        let run_rand_i = rng.gen_range(0..=run_length);
                        maze.connect_cells(x - run_rand_i, y, NORTH);

                        run_length = 0;
                    } else if y == 0 {
                        maze.connect_cells(x, y, EAST);
                    } else {
                        if rng.gen_range(0..=1) == 0 {
                            // Check length of run.
                            // Pick on randomly and erast north.
                            let run_rand_i = rng.gen_range(0..=run_length);
                            maze.connect_cells(x - run_rand_i, y, NORTH);

                            run_length = 0;
                        } else {
                            maze.connect_cells(x, y, EAST);
                            run_length += 1;
                        }
                    }
                }
            }
        }
    }

    #[allow(unused)]
    pub fn random_maze_creation(maze: &mut Maze, start: Pair<usize>) {
        let mut unreachable_cells: HashSet<Pair<usize>> = HashSet::new();
        for y in 0..maze.height {
            for x in 0..maze.width {
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
                    maze.neighbours(current_coord, CellReachType::UnreachableOnly);
                let mut neighbour_dirs = neighbour_coords.keys().collect::<Vec<_>>();

                let used_neighbour_count =
                    rnd.gen_range(min(1, neighbour_coords.len())..=min(2, neighbour_coords.len()));

                neighbour_dirs.shuffle(&mut rnd);
                for i in 0..used_neighbour_count {
                    maze.connect_cells(current_coord.x, current_coord.y, *neighbour_dirs[i]);
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
                    maze.neighbours(unreachable_cell, CellReachType::ReachableOnly);

                if !neighbour_coords.is_empty() {
                    let random_reachable_neighbour_dir = neighbour_coords.keys().next().unwrap();
                    maze.connect_cells(
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
}

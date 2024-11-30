use std::collections::HashSet;

use crate::util::*;
use crate::{Maze, Pair};

#[allow(unused)]
pub struct AsciiDrawer;

impl AsciiDrawer {
    #[allow(unused)]
    pub fn draw(maze: &Maze, solution: Vec<Pair<usize>>) {
        let solution_set: HashSet<Pair<usize>> = solution.into_iter().collect();

        for y in 0..maze.height {
            print!("█");
            for x in 0..maze.width {
                let i = y * maze.width + x;

                if maze.cells[i].paths[NORTH] {
                    print!("██");
                } else {
                    print!(" █");
                }
            }
            print!("\n");

            print!("█");
            for x in 0..maze.width {
                let i = y * maze.width + x;

                if solution_set.contains(&Pair::new(x, y)) {
                    print!("x");
                } else {
                    print!(" ");
                }

                if maze.cells[i].paths[EAST] {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            print!("\n")
        }

        print!("█");
        for _ in 0..maze.width {
            print!("██");
        }

        print!("\n\n");
    }
}

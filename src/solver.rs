use std::collections::VecDeque;

use crate::util::*;
use crate::{Maze, Pair};

pub struct Solver;

impl Solver {
    pub fn build_distance_map(maze: &Maze, start: Pair<usize>) -> (i32, Vec<Vec<i32>>) {
        let mut distance_map: Vec<Vec<i32>> = vec![vec![-1; maze.width]; maze.height];
        let mut max_distance = 0;

        let mut work_queue: VecDeque<Pair<usize>> = VecDeque::new();
        work_queue.push_back(start);
        distance_map[start.y][start.x] = 0;

        while let Some(current_coord) = work_queue.pop_front() {
            for dir in 0..4 {
                let neighbour_coord = Pair::new(
                    current_coord.x as i32 + NEIGHBOUR_MAP[dir][0],
                    current_coord.y as i32 + NEIGHBOUR_MAP[dir][1],
                );

                if neighbour_coord.x < 0
                    || neighbour_coord.y < 0
                    || neighbour_coord.x >= maze.width as i32
                    || neighbour_coord.y >= maze.height as i32
                {
                    continue;
                }

                let current_cell =
                    &maze.cells[(current_coord.y * maze.width + current_coord.x) as usize];
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
                max_distance = std::cmp::max(max_distance, current_distance + 1);
                let neighbour_coord_usize =
                    Pair::new(neighbour_coord.x as usize, neighbour_coord.y as usize);

                work_queue.push_back(neighbour_coord_usize);
            }
        }

        return (max_distance, distance_map);
    }

    #[allow(unused)]
    pub fn dijkstra_path_finding_solver(
        maze: &Maze,
        start: Pair<usize>,
        finish: Pair<usize>,
    ) -> Vec<Pair<usize>> {
        let mut distance_map: Vec<Vec<i32>> = vec![vec![-1; maze.width]; maze.height];

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
                    || neighbour_coord.x >= maze.width as i32
                    || neighbour_coord.y >= maze.height as i32
                {
                    continue;
                }

                let current_cell =
                    &maze.cells[(current_coord.y * maze.width + current_coord.x) as usize];
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
                    || neighbour_coord.x >= maze.width as i32
                    || neighbour_coord.y >= maze.height as i32
                {
                    continue;
                }

                if distance_map[neighbour_coord.y as usize][neighbour_coord.x as usize]
                    == current_distance - 1
                    && !maze.cells[current_coord.y * maze.width + current_coord.x].paths[dir]
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
}

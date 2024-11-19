use rand::prelude::*;

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

    fn binary_tree_maze_creation(&mut self) {
        let mut rng = rand::thread_rng();

        for y in 0..self.height {
            for x in 0..self.width {
                let i = y * self.width + x;

                if y == 0 && x == self.width - 1 {
                    continue;
                } else if y == 0 {
                    self.cells[i].paths[1] = false;
                } else if x == self.width - 1 {
                    self.cells[i].paths[0] = false;
                } else {
                    self.cells[i].paths[rng.gen_range(0..=1)] = false;
                }
            }
        }
    }

    fn dump(&self) {
        for y in 0..self.height {
            print!("+");
            for x in 0..self.width {
                let i = y * self.width + x;

                if self.cells[i].paths[0] {
                    print!("-+");
                } else {
                    print!(" +");
                }
            }
            print!("\n");

            print!("|");
            for x in 0..self.width {
                let i = y * self.width + x;

                if self.cells[i].paths[1] {
                    print!(" |");
                } else {
                    print!("  ");
                }
            }
            print!("\n")
        }

        print!("+");
        for x in 0..self.width {
            print!("-+");
        }

        print!("\n\n");
    }
}

fn main() {
    let mut maze = Maze::new_full(10, 6);
    maze.binary_tree_maze_creation();
    maze.dump();
}

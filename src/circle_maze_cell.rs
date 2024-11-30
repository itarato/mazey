#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CircleMazeCellDirection {
    North(usize),
    East,
    South,
    West,
}

#[derive(Clone, Debug)]
pub struct CircleMazeCell {
    paths: Vec<bool>,
    pub has_default_paths: bool, // Default paths: east + south + west.
    pub north_path_count: usize,
}

impl CircleMazeCell {
    pub fn new(has_default_paths: bool, north_path_count: usize) -> CircleMazeCell {
        let mut path_count = north_path_count;

        if has_default_paths {
            path_count += 3;
        }

        CircleMazeCell {
            paths: vec![true; path_count],
            has_default_paths,
            north_path_count,
        }
    }

    pub fn is_open_at(&self, dir: CircleMazeCellDirection) -> bool {
        match dir {
            CircleMazeCellDirection::East => !self.paths[0],
            CircleMazeCellDirection::South => !self.paths[1],
            CircleMazeCellDirection::West => !self.paths[2],
            CircleMazeCellDirection::North(n) => {
                let i = if self.has_default_paths { 3 } else { 0 };
                !self.paths[i + n]
            }
        }
    }

    pub fn open(&mut self, dir: CircleMazeCellDirection) {
        assert!(!self.is_open_at(dir.clone()));

        match dir {
            CircleMazeCellDirection::East => self.paths[0] = false,
            CircleMazeCellDirection::South => self.paths[1] = false,
            CircleMazeCellDirection::West => self.paths[2] = false,
            CircleMazeCellDirection::North(n) => {
                let i = if self.has_default_paths { 3 } else { 0 };
                self.paths[i + n] = false
            }
        };
    }

    pub fn reachable(&self) -> bool {
        self.paths.iter().any(|wall| !*wall)
    }
}

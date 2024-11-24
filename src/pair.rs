use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pair<T>
where
    T: Hash,
{
    pub x: T,
    pub y: T,
}

impl<T: Hash> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Pair<usize> {
    pub fn index(&self, width: usize) -> usize {
        self.y * width + self.x
    }
}

impl Pair<i32> {
    pub fn index(&self, width: usize) -> usize {
        self.y as usize * width + self.x as usize
    }

    pub fn to_usize(&self) -> Pair<usize> {
        Pair::new(self.x as usize, self.y as usize)
    }
}

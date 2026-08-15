use super::NumOps;
use super::Point;
use super::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Ray<T: NumOps, const K: usize> {
    origin: Point<T, K>,
    direction: Vector<T, K>
}

impl <T: NumOps, const K: usize> Ray<T, K> {
    pub fn origin(&self) -> Point<T, K> {
        self.origin
    }

    pub fn direction(&self) -> Vector<T, K> {
        self.direction
    }

    pub fn position(&self, step: T) -> Point<T, K> {
        self.origin + self.direction * step
    }
}

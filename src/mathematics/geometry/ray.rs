use crate::mathematics::traits::Number;
use crate::mathematics::geometry::Point;
use crate::mathematics::linear_algebra::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Ray<T: Number, const N: usize> {
    origin: Point<T, N>,
    direction: Vector<T, N>,
}

impl<T: Number, const N: usize> Ray<T, N> {
    pub fn new(origin: Point<T, N>, direction: Vector<T, N>) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point<T, N> {
        self.origin
    }

    pub fn direction(&self) -> Vector<T, N> {
        self.direction
    }

    pub fn position(&self, step: T) -> Point<T, N> {
        self.origin + self.direction * step
    }
}

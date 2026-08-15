use super::Point;
use super::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Ray<T, const K: usize> {
    origin: Point<T, K>,
    direction: Vector<T, K>
}

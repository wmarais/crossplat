use core::ops::{Add, Sub, Mul, Div, Index, IndexMut};

use super::NumOps;
use super::Vector;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Point<T: NumOps, const K: usize>([T; K]);

impl<T: NumOps, const K: usize> Point<T, K> {
    pub fn new(values: [T;K]) -> Self {
        Self(values)
    }
}

impl<T: NumOps, const K: usize> Index<usize> for Point<T, K> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: NumOps, const K: usize> IndexMut<usize> for Point<T, K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: NumOps, const K: usize> Add<Vector<T, K>> for Point<T, K> {
    type Output = Self;
    fn add(self, rhs: Vector<T, K>) -> Self::Output {
        Self(core::array::from_fn(|i| self[i] + rhs[i]))
    }
}

impl<T: NumOps, const K: usize> Sub<Vector<T, K>> for Point<T, K> {
    type Output = Self;
    fn sub(self, rhs: Vector<T, K>) -> Self::Output {
        Self(core::array::from_fn(|i| self[i] - rhs[i]))
    }
}

impl<T: NumOps, const K: usize> Sub<Point<T, K>> for Point<T, K> {
    type Output = Vector<T, K>;
    fn sub(self, rhs: Point<T, K>) -> Self::Output {
        Vector::new(core::array::from_fn(|i| self[i] - rhs[i]))
    }
}

impl<T: NumOps, const K: usize> Mul<T> for Point<T, K> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self(core::array::from_fn(|i| self[i] * rhs))
    }
}

impl<T: NumOps, const K: usize> Div<T> for Point<T, K> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self(core::array::from_fn(|i| self[i] / rhs))
    }
}


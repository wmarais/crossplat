use core::ops::{Add, Sub, Mul, Div, Index, IndexMut};

use super::NumOps;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Vector<T: NumOps, const K: usize>([T; K]);

impl<T: NumOps, const K: usize> Vector<T, K> {

    pub fn length(&self) -> T {
        let mut len: T = T::zero();
        for k in 0..K {
            len = len + self.0[k] * self.0[k]
        }
        len.sqrt()
    }

    pub fn normal(&self) -> Self {
        *self/self.length()
    }
}

impl<T: NumOps, const K: usize> Index<usize> for Vector<T, K> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: NumOps, const K: usize> IndexMut<usize> for Vector<T, K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: NumOps, const K: usize> Add<Vector<T, K>> for Vector<T, K> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output{ 0: core::array::from_fn(|i| self.0[i] + rhs.0[i]) }
    }
}

impl<T: NumOps, const K: usize> Sub<Vector<T, K>> for Vector<T, K> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output{ 0: core::array::from_fn(|i| self.0[i] + rhs.0[i]) }
    }
}

impl<T: NumOps, const K: usize> Mul<T> for Vector<T, K> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::Output{ 0: core::array::from_fn(|i| self.0[i] * rhs) }
    }
}

impl<T: NumOps, const K: usize> Div<T> for Vector<T, K> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::Output{ 0: core::array::from_fn(|i| self.0[i] / rhs) }
    }
}

use core::ops::{Add, Sub, Mul, Div, Index, IndexMut};

use super::NumOps;
use super::Point;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector<T: NumOps, const K: usize>([T; K]);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ZeroLengthVector;

impl<T: NumOps, const K: usize> Vector<T, K> {
    pub fn new(values: [T;K]) -> Self {
        Self(values)
    }

    pub fn zero() -> Self {
        Self::new([T::zero(); K])
    }

    pub fn length(&self) -> T {
        let mut len: T = T::zero();
        for k in 0..K {
            len = len + self.0[k] * self.0[k]
        }
        len.sqrt()
    }

    pub fn normal(&self) -> Result<Self, ZeroLengthVector> {
        let len = self.length();
        if len == T::zero() {
            Err(ZeroLengthVector)
        }
        else {
            Ok(*self/len)
        }
    }

    pub fn dot(&self, rhs: &Vector<T, K>) -> T {
        let mut result: T = T::zero();
        for k in 0..K {
            result = result + self[k] * rhs[k];
        }
        result
    }

}

impl<T: NumOps, const K: usize> From<Point<T, K>> for Vector<T, K> {
    fn from(value: Point<T, K>) -> Self {
        let mut result = Vector::new([T::zero(); K]);
        for k in 0..K {
            result[k] = value[k];
        }
        result
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
        Self(core::array::from_fn(|i| self[i] + rhs[i]))
    }
}

impl<T: NumOps, const K: usize> Sub<Vector<T, K>> for Vector<T, K> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(core::array::from_fn(|i| self[i] - rhs[i]))
    }
}

impl<T: NumOps, const K: usize> Mul<T> for Vector<T, K> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self(core::array::from_fn(|i| self[i] * rhs))
    }
}

impl<T: NumOps, const K: usize> Div<T> for Vector<T, K> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self(core::array::from_fn(|i| self[i] / rhs))
    }
}

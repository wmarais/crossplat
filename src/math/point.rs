use core::ops::{Add, Sub, Mul, Div, Index, IndexMut};

use super::NumOps;
use super::Vector;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_index() {
        let p = Point::new([1.0, 2.0, 3.0]);
        assert_eq!(p[0], 1.0);
        assert_eq!(p[1], 2.0);
        assert_eq!(p[2], 3.0);
    }

    #[test]
    fn index_mut_sets_value() {
        let mut p = Point::new([0.0, 0.0]);
        p[0] = 5.0;
        p[1] = -3.0;
        assert_eq!(p, Point::new([5.0, -3.0]));
    }

    #[test]
    fn add_vector_translates_point() {
        let p = Point::new([1.0, 2.0]);
        let v = Vector::new([3.0, 4.0]);
        assert_eq!(p + v, Point::new([4.0, 6.0]));
    }

    #[test]
    fn sub_vector_translates_point() {
        let p = Point::new([5.0, 6.0]);
        let v = Vector::new([1.0, 2.0]);
        assert_eq!(p - v, Point::new([4.0, 4.0]));
    }

    #[test]
    fn sub_point_yields_displacement_vector_not_point() {
        // Regression: this used to return a Point (Self) instead of a Vector.
        let a = Point::new([5.0, 7.0]);
        let b = Point::new([2.0, 3.0]);
        let d: Vector<f64, 2> = a - b;
        assert_eq!(d, Vector::new([3.0, 4.0]));
    }

    #[test]
    fn mul_scales_components() {
        let p = Point::new([1.0, -2.0]);
        assert_eq!(p * 3.0, Point::new([3.0, -6.0]));
    }

    #[test]
    fn div_scales_components() {
        let p = Point::new([6.0, -9.0]);
        assert_eq!(p / 3.0, Point::new([2.0, -3.0]));
    }
}


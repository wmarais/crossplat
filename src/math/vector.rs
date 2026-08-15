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

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn new_and_index() {
        let v = Vector::new([1.0, 2.0, 3.0]);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn index_mut_sets_value() {
        let mut v = Vector::<f64, 2>::zero();
        v[0] = 5.0;
        v[1] = -2.0;
        assert_eq!(v, Vector::new([5.0, -2.0]));
    }

    #[test]
    fn zero_is_all_zero() {
        let v = Vector::<f64, 4>::zero();
        assert_eq!(v, Vector::new([0.0, 0.0, 0.0, 0.0]));
    }

    #[test]
    fn length_matches_known_triangle() {
        let v = Vector::new([3.0, 4.0]);
        assert!(approx(v.length(), 5.0));
    }

    #[test]
    fn length_of_zero_vector_is_zero() {
        let v = Vector::<f64, 3>::zero();
        assert_eq!(v.length(), 0.0);
    }

    #[test]
    fn normal_has_unit_length_and_matches_known_direction() {
        let v = Vector::new([3.0, 4.0]);
        let n = v.normal().unwrap();
        assert!(approx(n[0], 0.6));
        assert!(approx(n[1], 0.8));
        assert!(approx(n.length(), 1.0));
    }

    #[test]
    fn normal_of_zero_vector_errs() {
        let v = Vector::<f64, 3>::zero();
        assert_eq!(v.normal(), Err(ZeroLengthVector));
    }

    #[test]
    fn dot_product_known_value() {
        let a = Vector::new([1.0, 2.0, 3.0]);
        let b = Vector::new([4.0, 5.0, 6.0]);
        assert_eq!(a.dot(&b), 32.0);
    }

    #[test]
    fn dot_product_orthogonal_is_zero() {
        let a = Vector::new([1.0, 0.0]);
        let b = Vector::new([0.0, 1.0]);
        assert_eq!(a.dot(&b), 0.0);
    }

    #[test]
    fn from_point_copies_components() {
        let p = Point::new([1.0, 2.0, 3.0]);
        let v = Vector::from(p);
        assert_eq!(v, Vector::new([1.0, 2.0, 3.0]));
    }

    #[test]
    fn add_sums_components() {
        let a = Vector::new([1.0, 2.0]);
        let b = Vector::new([3.0, 4.0]);
        assert_eq!(a + b, Vector::new([4.0, 6.0]));
    }

    #[test]
    fn sub_subtracts_components() {
        let a = Vector::new([5.0, 7.0]);
        let b = Vector::new([2.0, 3.0]);
        assert_eq!(a - b, Vector::new([3.0, 4.0]));
        assert_ne!(a - b, a + b);
    }

    #[test]
    fn mul_scales_components() {
        let a = Vector::new([1.0, -2.0, 3.0]);
        assert_eq!(a * 2.0, Vector::new([2.0, -4.0, 6.0]));
    }

    #[test]
    fn div_scales_components() {
        let a = Vector::new([2.0, -4.0, 6.0]);
        assert_eq!(a / 2.0, Vector::new([1.0, -2.0, 3.0]));
    }
}

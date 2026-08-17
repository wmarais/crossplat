use core::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use crate::mathematics::geometry::Point;
use crate::mathematics::traits::Number;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector<T: Number, const N: usize>([T; N]);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ZeroLengthVector;

impl<T: Number, const N: usize> Vector<T, N> {
    pub fn new(values: [T; N]) -> Self {
        Self(values)
    }

    pub fn zero() -> Self {
        Self::new([T::zero(); N])
    }

    /// Bounds-checked component access. Prefer this over `[]` when `index`
    /// comes from external/untrusted input, since `[]` panics out of bounds.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    /// Bounds-checked mutable component access. Prefer this over `[]` when
    /// `index` comes from external/untrusted input, since `[]` panics out of
    /// bounds.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.0.get_mut(index)
    }

    pub fn length(&self) -> T {
        let mut len: T = T::zero();
        for n in 0..N {
            len = len + self.0[n] * self.0[n]
        }
        len.sqrt()
    }

    pub fn normal(&self) -> Result<Self, ZeroLengthVector> {
        self.checked_div(self.length()).ok_or(ZeroLengthVector)
    }

    /// Divides by `rhs`, returning `None` instead of producing `inf`/`NaN`
    /// (or panicking, for integer `T`) when `rhs` is zero.
    pub fn checked_div(&self, rhs: T) -> Option<Self> {
        if rhs.approx_equal_small(&T::zero()) {
            None
        } else {
            Some(*self / rhs)
        }
    }

    /// In-place version of [`Self::checked_div`]. Leaves `self` unmodified
    /// and returns `None` when `rhs` is zero.
    pub fn checked_div_assign(&mut self, rhs: T) -> Option<()> {
        if rhs.approx_equal_small(&T::zero()) {
            None
        } else {
            *self /= rhs;
            Some(())
        }
    }

    pub fn dot(&self, rhs: &Vector<T, N>) -> T {
        let mut result: T = T::zero();
        for n in 0..N {
            result = result + self[n] * rhs[n];
        }
        result
    }
}

impl<T: Number> Vector<T, 3> {
    pub fn cross(&self, rhs: &Vector<T, 3>) -> Self {
        Self::new([
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        ])
    }
}

impl<T: Number, const N: usize> From<Point<T, N>> for Vector<T, N> {
    fn from(value: Point<T, N>) -> Self {
        let mut result = Vector::new([T::zero(); N]);
        for n in 0..N {
            result[n] = value[n];
        }
        result
    }
}

/// Panics if `index` is out of bounds. Use [`Vector::get`] instead when
/// `index` comes from external/untrusted input.
impl<T: Number, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

/// Panics if `index` is out of bounds. Use [`Vector::get_mut`] instead when
/// `index` comes from external/untrusted input.
impl<T: Number, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Number, const N: usize> Add<Vector<T, N>> for Vector<T, N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(core::array::from_fn(|i| self[i] + rhs[i]))
    }
}

impl<T: Number, const N: usize> AddAssign<Vector<T, N>> for Vector<T, N> {
    fn add_assign(&mut self, rhs: Vector<T, N>) {
        for n in 0..N {
            self[n] += rhs[n];
        }
    }
}

impl<T: Number, const N: usize> Sub<Vector<T, N>> for Vector<T, N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(core::array::from_fn(|i| self[i] - rhs[i]))
    }
}

impl<T: Number, const N: usize> SubAssign<Vector<T, N>> for Vector<T, N> {
    fn sub_assign(&mut self, rhs: Vector<T, N>) {
        for n in 0..N {
            self[n] -= rhs[n];
        }
    }
}

impl<T: Number, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self(core::array::from_fn(|i| self[i] * rhs))
    }
}

impl<T: Number, const N: usize> MulAssign<T> for Vector<T, N> {
    fn mul_assign(&mut self, rhs: T) {
        for n in 0..N {
            self[n] *= rhs;
        }
    }
}

/// For integer `T` this panics on division by zero; for float `T` it
/// silently produces `inf`/`NaN`. Use [`Vector::checked_div`] instead when
/// `rhs` comes from external/untrusted input.
impl<T: Number, const N: usize> Div<T> for Vector<T, N> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self(core::array::from_fn(|i| self[i] / rhs))
    }
}

/// For integer `T` this panics on division by zero; for float `T` it
/// silently produces `inf`/`NaN`. Use [`Vector::checked_div_assign`] instead
/// when `rhs` comes from external/untrusted input.
impl<T: Number, const N: usize> DivAssign<T> for Vector<T, N> {
    fn div_assign(&mut self, rhs: T) {
        for n in 0..N {
            self[n] /= rhs;
        }
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
    fn get_returns_some_for_valid_index() {
        let v = Vector::new([1.0, 2.0, 3.0]);
        assert_eq!(v.get(0), Some(&1.0));
        assert_eq!(v.get(2), Some(&3.0));
    }

    #[test]
    fn get_returns_none_for_out_of_bounds_index() {
        let v = Vector::new([1.0, 2.0, 3.0]);
        assert_eq!(v.get(3), None);
        assert_eq!(v.get(usize::MAX), None);
    }

    #[test]
    fn get_mut_returns_some_and_allows_mutation() {
        let mut v = Vector::new([1.0, 2.0, 3.0]);
        if let Some(component) = v.get_mut(1) {
            *component = 9.0;
        }
        assert_eq!(v, Vector::new([1.0, 9.0, 3.0]));
    }

    #[test]
    fn get_mut_returns_none_for_out_of_bounds_index() {
        let mut v = Vector::new([1.0, 2.0, 3.0]);
        assert_eq!(v.get_mut(3), None);
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
    fn cross_product_of_unit_axes_matches_known_value() {
        let x = Vector::new([1.0, 0.0, 0.0]);
        let y = Vector::new([0.0, 1.0, 0.0]);
        assert_eq!(x.cross(&y), Vector::new([0.0, 0.0, 1.0]));
    }

    #[test]
    fn cross_product_is_orthogonal_to_both_inputs() {
        let a = Vector::new([2.0, 3.0, 4.0]);
        let b = Vector::new([5.0, 6.0, 7.0]);
        let c = a.cross(&b);
        assert!(approx(c.dot(&a), 0.0));
        assert!(approx(c.dot(&b), 0.0));
    }

    #[test]
    fn cross_product_of_parallel_vectors_is_zero() {
        let a = Vector::new([1.0, 2.0, 3.0]);
        let b = Vector::new([2.0, 4.0, 6.0]);
        assert_eq!(a.cross(&b), Vector::zero());
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

    #[test]
    fn add_assign_sums_components() {
        let mut a = Vector::new([1.0, 2.0]);
        let b = Vector::new([3.0, 4.0]);
        a += b;
        assert_eq!(a, Vector::new([4.0, 6.0]));
    }

    #[test]
    fn sub_assign_subtracts_components() {
        let mut a = Vector::new([5.0, 7.0]);
        let b = Vector::new([2.0, 3.0]);
        a -= b;
        assert_eq!(a, Vector::new([3.0, 4.0]));
    }

    #[test]
    fn mul_assign_scales_components() {
        let mut a = Vector::new([1.0, -2.0, 3.0]);
        a *= 2.0;
        assert_eq!(a, Vector::new([2.0, -4.0, 6.0]));
    }

    #[test]
    fn div_assign_scales_components() {
        let mut a = Vector::new([2.0, -4.0, 6.0]);
        a /= 2.0;
        assert_eq!(a, Vector::new([1.0, -2.0, 3.0]));
    }

    #[test]
    fn checked_div_scales_components() {
        let a = Vector::new([2.0, -4.0, 6.0]);
        assert_eq!(a.checked_div(2.0), Some(Vector::new([1.0, -2.0, 3.0])));
    }

    #[test]
    fn checked_div_by_zero_is_none() {
        let a = Vector::new([1.0, 2.0, 3.0]);
        assert_eq!(a.checked_div(0.0), None);
    }

    #[test]
    fn checked_div_assign_scales_components() {
        let mut a = Vector::new([2.0, -4.0, 6.0]);
        assert_eq!(a.checked_div_assign(2.0), Some(()));
        assert_eq!(a, Vector::new([1.0, -2.0, 3.0]));
    }

    #[test]
    fn checked_div_assign_by_zero_is_none_and_leaves_value_unmodified() {
        let mut a = Vector::new([1.0, 2.0, 3.0]);
        assert_eq!(a.checked_div_assign(0.0), None);
        assert_eq!(a, Vector::new([1.0, 2.0, 3.0]));
    }
}

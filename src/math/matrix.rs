use super::NumOps;
use super::Point;
use super::Vector;
use core::ops::{Add, Div, Index, IndexMut, Mul, Sub};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Matrix<T: NumOps, const M: usize, const N: usize>([Vector<T, N>; M]);

impl<T: NumOps, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn new(rows: [Vector<T, N>; M]) -> Self {
        Self(rows)
    }

    pub fn zero() -> Self {
        Self([Vector::new([T::zero(); N]); M])
    }

    pub fn row(&self, index: usize) -> Vector<T, N> {
        self[index]
    }

    pub fn column(&self, index: usize) -> Vector<T, M> {
        let mut result = Vector::<T, M>::zero();
        for m in 0..M {
            result[m] = self[m][index];
        }
        result
    }
}

impl<T: NumOps, const M: usize> Matrix<T, M, M> {
    pub fn determinant(&self) -> T {
        let mut mat = *self;
        let mut swaps: usize = 0;

        for k in 0..M {
            if mat[k][k].approx_equal_small(&T::zero()) {
                match (k + 1..M).find(|&i| !mat[i][k].approx_equal_small(&T::zero())) {
                    Some(i) => {
                        let tmp = mat[k];
                        mat[k] = mat[i];
                        mat[i] = tmp;
                        swaps += 1;
                    }
                    None => return T::zero(),
                }
            }

            for i in (k + 1)..M {
                let factor = mat[i][k] / mat[k][k];
                mat[i] = mat[i] - mat[k] * factor;
            }
        }

        let mut det = mat[0][0];
        for k in 1..M {
            det = det * mat[k][k];
        }

        if swaps % 2 == 1 {
            det = T::zero() - det;
        }

        det
    }
}

impl<T: NumOps, const M: usize, const N: usize> Index<usize> for Matrix<T, M, N> {
    type Output = Vector<T, N>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: NumOps, const M: usize, const N: usize> IndexMut<usize> for Matrix<T, M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: NumOps, const M: usize, const N: usize> Mul<T> for Matrix<T, M, N> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self(core::array::from_fn(|m| self[m] * rhs))
    }
}

impl<T: NumOps, const M: usize, const N: usize> Mul<Vector<T, N>> for Matrix<T, M, N> {
    type Output = Vector<T, M>;
    fn mul(self, rhs: Vector<T, N>) -> Self::Output {
        let mut result = Self::Output::new([T::zero(); M]);
        for m in 0..M {
            result[m] = self[m].dot(&rhs);
        }
        result
    }
}

impl<T: NumOps, const M: usize, const N: usize> Mul<Point<T, N>> for Matrix<T, M, N> {
    type Output = Point<T, M>;
    fn mul(self, rhs: Point<T, N>) -> Self::Output {
        let mut result = Self::Output::new([T::zero(); M]);
        let rhs_vec = Vector::from(rhs);
        for m in 0..M {
            result[m] = self[m].dot(&rhs_vec);
        }
        result
    }
}

impl<T: NumOps, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, N, P>>
    for Matrix<T, M, N>
{
    type Output = Matrix<T, M, P>;
    fn mul(self, rhs: Matrix<T, N, P>) -> Self::Output {
        let rows: [Vector<T, N>; M] = core::array::from_fn(|m| self.row(m));
        let cols: [Vector<T, N>; P] = core::array::from_fn(|p| rhs.column(p));
        let mut result = Self::Output::zero();
        for m in 0..M {
            for p in 0..P {
                result[m][p] = rows[m].dot(&cols[p]);
            }
        }
        result
    }
}

impl<T: NumOps, const M: usize, const N: usize> Div<T> for Matrix<T, M, N> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self(core::array::from_fn(|m| self[m] / rhs))
    }
}

impl<T: NumOps, const M: usize, const N: usize> Add<Matrix<T, M, N>> for Matrix<T, M, N> {
    type Output = Self;
    fn add(self, rhs: Matrix<T, M, N>) -> Self::Output {
        Self(core::array::from_fn(|m| self[m] + rhs[m]))
    }
}

impl<T: NumOps, const M: usize, const N: usize> Sub<Matrix<T, M, N>> for Matrix<T, M, N> {
    type Output = Self;
    fn sub(self, rhs: Matrix<T, M, N>) -> Self::Output {
        Self(core::array::from_fn(|m| self[m] - rhs[m]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn zero_matrix_is_all_zero() {
        let m: Matrix<f64, 2, 3> = Matrix::zero();
        for i in 0..2 {
            for j in 0..3 {
                assert_eq!(m[i][j], 0.0);
            }
        }
    }

    #[test]
    fn new_and_index() {
        let m = Matrix::new([Vector::new([1.0, 2.0, 3.0]), Vector::new([4.0, 5.0, 6.0])]);
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[1][2], 6.0);
    }

    #[test]
    fn index_mut_sets_value() {
        let mut m: Matrix<f64, 2, 2> = Matrix::zero();
        m[0][1] = 7.0;
        assert_eq!(m[0][1], 7.0);
    }

    #[test]
    fn row_returns_full_row() {
        let m = Matrix::new([Vector::new([1.0, 2.0]), Vector::new([3.0, 4.0])]);
        assert_eq!(m.row(1), Vector::new([3.0, 4.0]));
    }

    #[test]
    fn column_returns_full_column() {
        let m = Matrix::new([Vector::new([1.0, 2.0, 3.0]), Vector::new([4.0, 5.0, 6.0])]);
        assert_eq!(m.column(1), Vector::new([2.0, 5.0]));
    }

    #[test]
    fn scalar_mul_scales_every_entry() {
        let m = Matrix::new([Vector::new([1.0, 2.0]), Vector::new([3.0, 4.0])]);
        let expected = Matrix::new([Vector::new([2.0, 4.0]), Vector::new([6.0, 8.0])]);
        assert_eq!(m * 2.0, expected);
    }

    #[test]
    fn scalar_div_scales_every_entry() {
        let m = Matrix::new([Vector::new([2.0, 4.0]), Vector::new([6.0, 8.0])]);
        let expected = Matrix::new([Vector::new([1.0, 2.0]), Vector::new([3.0, 4.0])]);
        assert_eq!(m / 2.0, expected);
    }

    #[test]
    fn add_sums_matching_entries() {
        let a = Matrix::new([Vector::new([1.0, 2.0]), Vector::new([3.0, 4.0])]);
        let b = Matrix::new([Vector::new([5.0, 6.0]), Vector::new([7.0, 8.0])]);
        let expected = Matrix::new([Vector::new([6.0, 8.0]), Vector::new([10.0, 12.0])]);
        assert_eq!(a + b, expected);
    }

    #[test]
    fn sub_subtracts_matching_entries() {
        let a = Matrix::new([Vector::new([5.0, 6.0]), Vector::new([7.0, 8.0])]);
        let b = Matrix::new([Vector::new([1.0, 2.0]), Vector::new([3.0, 4.0])]);
        let expected = Matrix::new([Vector::new([4.0, 4.0]), Vector::new([4.0, 4.0])]);
        assert_eq!(a - b, expected);
    }

    #[test]
    fn mul_vector_output_len_matches_row_count_non_square() {
        // Regression: a 2x3 matrix times a 3-vector must yield a 2-vector, not
        // an N-sized (3-element) result.
        let m = Matrix::new([Vector::new([1.0, 2.0, 3.0]), Vector::new([4.0, 5.0, 6.0])]);
        let v = Vector::new([1.0, 1.0, 1.0]);
        assert_eq!(m * v, Vector::new([6.0, 15.0]));
    }

    #[test]
    fn mul_point_output_len_matches_row_count_non_square() {
        let m = Matrix::new([Vector::new([1.0, 0.0, 0.0]), Vector::new([0.0, 1.0, 0.0])]);
        let p = Point::new([2.0, 3.0, 4.0]);
        assert_eq!(m * p, Point::new([2.0, 3.0]));
    }

    #[test]
    fn matrix_mul_matrix_non_square() {
        // (2x3) * (3x2) = (2x2)
        let a = Matrix::new([Vector::new([1.0, 2.0, 3.0]), Vector::new([4.0, 5.0, 6.0])]);
        let b = Matrix::new([
            Vector::new([7.0, 8.0]),
            Vector::new([9.0, 10.0]),
            Vector::new([11.0, 12.0]),
        ]);
        let expected = Matrix::new([Vector::new([58.0, 64.0]), Vector::new([139.0, 154.0])]);
        assert_eq!(a * b, expected);
    }

    #[test]
    fn determinant_2x2() {
        let m = Matrix::new([Vector::new([3.0, 8.0]), Vector::new([4.0, 6.0])]);
        assert!(approx(m.determinant(), -14.0));
    }

    #[test]
    fn determinant_requires_row_swap() {
        let m = Matrix::new([Vector::new([0.0, 1.0]), Vector::new([1.0, 0.0])]);
        assert!(approx(m.determinant(), -1.0));
    }

    #[test]
    fn determinant_3x3() {
        let m = Matrix::new([
            Vector::new([6.0, 1.0, 1.0]),
            Vector::new([4.0, -2.0, 5.0]),
            Vector::new([2.0, 8.0, 7.0]),
        ]);
        assert!(approx(m.determinant(), -306.0));
    }

    #[test]
    fn determinant_of_singular_matrix_is_zero() {
        let m = Matrix::new([
            Vector::new([1.0, 2.0, 3.0]),
            Vector::new([2.0, 4.0, 6.0]),
            Vector::new([1.0, 0.0, 1.0]),
        ]);
        assert!(approx(m.determinant(), 0.0));
    }
}

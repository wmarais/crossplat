use core::ops::{Add, Sub, Mul, Div, Index, IndexMut};
use super::NumOps;
use super::Vector;
use super::Point;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Matrix<T: NumOps, const M: usize, const N: usize>([Vector<T, N>; M]);

impl<T: NumOps, const M: usize, const N: usize> Matrix<T, M, N> {
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

impl<T: NumOps, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, N, P>> for Matrix<T, M, N> {
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




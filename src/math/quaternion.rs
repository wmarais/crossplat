use super::Matrix;
use super::NumOps;
use super::Vector;
use core::ops::{Index, IndexMut};

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Quaternion<T: NumOps>([T; 4]);

impl<T: NumOps> Quaternion<T> {
    // Storage order is (x, y, z, w). Angles are radians; x = roll, y = pitch,
    // z = yaw, combined in that order (intrinsic Z-Y-X / yaw-pitch-roll).
    pub fn from_euler(x: T, y: T, z: T) -> Self {
        let two = T::one() + T::one();

        let (sr, cr) = ((x / two).sin(), (x / two).cos());
        let (sp, cp) = ((y / two).sin(), (y / two).cos());
        let (sy, cy) = ((z / two).sin(), (z / two).cos());

        let w = cr * cp * cy + sr * sp * sy;
        let qx = sr * cp * cy - cr * sp * sy;
        let qy = cr * sp * cy + sr * cp * sy;
        let qz = cr * cp * sy - sr * sp * cy;

        Self([qx, qy, qz, w])
    }

    pub fn to_matrix(&self) -> Matrix<T, 4, 4> {
        let x = self[0];
        let y = self[1];
        let z = self[2];
        let w = self[3];

        let one = T::one();
        let two = one + one;

        let xx = x * x;
        let yy = y * y;
        let zz = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let wx = w * x;
        let wy = w * y;
        let wz = w * z;

        Matrix::new([
            Vector::new([
                one - two * (yy + zz),
                two * (xy - wz),
                two * (xz + wy),
                T::zero(),
            ]),
            Vector::new([
                two * (xy + wz),
                one - two * (xx + zz),
                two * (yz - wx),
                T::zero(),
            ]),
            Vector::new([
                two * (xz - wy),
                two * (yz + wx),
                one - two * (xx + yy),
                T::zero(),
            ]),
            Vector::new([T::zero(), T::zero(), T::zero(), one]),
        ])
    }
}

impl<T: NumOps> Index<usize> for Quaternion<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: NumOps> IndexMut<usize> for Quaternion<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

use super::Matrix;
use super::NumOps;
use super::Vector;
use core::ops::{Index, IndexMut, Mul};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Quaternion<T: NumOps>([T; 4]);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ZeroLengthQuaternion;

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

    pub fn rotate_about_axis(angle: T, axis: Vector<T, 4>) -> Self {
        let ax = axis[0];
        let ay = axis[1];
        let az = axis[2];

        let mag = (ax * ax + ay * ay + az * az).sqrt();
        let (nx, ny, nz) = (ax / mag, ay / mag, az / mag);

        let two = T::one() + T::one();
        let (s, c) = ((angle / two).sin(), (angle / two).cos());

        Self([nx * s, ny * s, nz * s, c])
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

    pub fn conjugate(&self) -> Self {
        let zero = T::zero();
        Self([zero - self[0], zero - self[1], zero - self[2], self[3]])
    }

    pub fn length(&self) -> T {
        let mut len = T::zero();
        for k in 0..4 {
            len = len + self[k] * self[k];
        }
        len.sqrt()
    }

    pub fn normal(&self) -> Result<Self, ZeroLengthQuaternion> {
        let len = self.length();
        if len.approx_equal_small(&T::zero()) {
            Err(ZeroLengthQuaternion)
        } else {
            Ok(Self([
                self[0] / len,
                self[1] / len,
                self[2] / len,
                self[3] / len,
            ]))
        }
    }

    pub fn inverse(&self) -> Result<Self, ZeroLengthQuaternion> {
        let mut len_sq = T::zero();
        for k in 0..4 {
            len_sq = len_sq + self[k] * self[k];
        }

        if len_sq.approx_equal_small(&T::zero()) {
            Err(ZeroLengthQuaternion)
        } else {
            let conj = self.conjugate();
            Ok(Self([
                conj[0] / len_sq,
                conj[1] / len_sq,
                conj[2] / len_sq,
                conj[3] / len_sq,
            ]))
        }
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

impl<T: NumOps> Mul<Quaternion<T>> for Quaternion<T> {
    type Output = Self;
    fn mul(self, rhs: Quaternion<T>) -> Self::Output {
        let (x1, y1, z1, w1) = (self[0], self[1], self[2], self[3]);
        let (x2, y2, z2, w2) = (rhs[0], rhs[1], rhs[2], rhs[3]);

        let w = w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2;
        let x = w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2;
        let y = w1 * y2 - x1 * z2 + y1 * w2 + z1 * x2;
        let z = w1 * z2 + x1 * y2 - y1 * x2 + z1 * w2;

        Self([x, y, z, w])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn approx_quat(a: Quaternion<f64>, b: Quaternion<f64>) -> bool {
        (0..4).all(|k| approx(a[k], b[k]))
    }

    fn with_components(x: f64, y: f64, z: f64, w: f64) -> Quaternion<f64> {
        let mut q = Quaternion::<f64>::from_euler(0.0, 0.0, 0.0);
        q[0] = x;
        q[1] = y;
        q[2] = z;
        q[3] = w;
        q
    }

    #[test]
    fn from_euler_identity_is_identity_quaternion() {
        let q = Quaternion::<f64>::from_euler(0.0, 0.0, 0.0);
        assert!(approx_quat(q, with_components(0.0, 0.0, 0.0, 1.0)));
    }

    #[test]
    fn from_euler_z_rotation_matches_known_value() {
        let q = Quaternion::<f64>::from_euler(0.0, 0.0, PI / 2.0);
        let half = (PI / 4.0).sin();
        assert!(approx_quat(q, with_components(0.0, 0.0, half, half)));
    }

    #[test]
    fn from_euler_produces_unit_quaternion() {
        let q = Quaternion::<f64>::from_euler(0.3, 0.5, 0.7);
        assert!(approx(q.length(), 1.0));
    }

    #[test]
    fn rotate_about_axis_matches_from_euler_for_x_axis() {
        let via_axis =
            Quaternion::<f64>::rotate_about_axis(PI / 2.0, Vector::new([1.0, 0.0, 0.0, 0.0]));
        let via_euler = Quaternion::<f64>::from_euler(PI / 2.0, 0.0, 0.0);
        assert!(approx_quat(via_axis, via_euler));
    }

    #[test]
    fn rotate_about_axis_matches_from_euler_for_z_axis() {
        let via_axis =
            Quaternion::<f64>::rotate_about_axis(PI / 3.0, Vector::new([0.0, 0.0, 1.0, 0.0]));
        let via_euler = Quaternion::<f64>::from_euler(0.0, 0.0, PI / 3.0);
        assert!(approx_quat(via_axis, via_euler));
    }

    #[test]
    fn rotate_about_axis_normalizes_non_unit_axis() {
        let scaled =
            Quaternion::<f64>::rotate_about_axis(PI / 2.0, Vector::new([5.0, 0.0, 0.0, 0.0]));
        let unit =
            Quaternion::<f64>::rotate_about_axis(PI / 2.0, Vector::new([1.0, 0.0, 0.0, 0.0]));
        assert!(approx_quat(scaled, unit));
        assert!(approx(scaled.length(), 1.0));
    }

    #[test]
    fn to_matrix_identity_is_identity_matrix() {
        let q = Quaternion::<f64>::from_euler(0.0, 0.0, 0.0);
        let m = q.to_matrix();
        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(approx(m[i][j], expected));
            }
        }
    }

    #[test]
    fn to_matrix_z90_matches_known_rotation_matrix() {
        let q = Quaternion::<f64>::from_euler(0.0, 0.0, PI / 2.0);
        let m = q.to_matrix();
        let expected = [
            [0.0, -1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        for i in 0..4 {
            for j in 0..4 {
                assert!(approx(m[i][j], expected[i][j]));
            }
        }
    }

    #[test]
    fn conjugate_negates_vector_part_and_keeps_scalar() {
        let q = with_components(1.0, 2.0, 3.0, 4.0);
        let c = q.conjugate();
        assert_eq!((c[0], c[1], c[2], c[3]), (-1.0, -2.0, -3.0, 4.0));
    }

    #[test]
    fn length_matches_known_value() {
        let q = with_components(1.0, 2.0, 3.0, 4.0);
        assert!(approx(q.length(), 30.0f64.sqrt()));
    }

    #[test]
    fn normal_of_non_unit_quaternion_has_unit_length() {
        let q = with_components(1.0, 2.0, 3.0, 4.0);
        let n = q.normal().unwrap();
        assert!(approx(n.length(), 1.0));
    }

    #[test]
    fn normal_of_zero_quaternion_errs() {
        let q = with_components(0.0, 0.0, 0.0, 0.0);
        assert_eq!(q.normal(), Err(ZeroLengthQuaternion));
    }

    #[test]
    fn inverse_is_two_sided_inverse_for_unit_quaternion() {
        let q = Quaternion::<f64>::from_euler(0.3, 0.5, 0.7);
        let inv = q.inverse().unwrap();
        let identity = with_components(0.0, 0.0, 0.0, 1.0);
        assert!(approx_quat(q * inv, identity));
        assert!(approx_quat(inv * q, identity));
    }

    #[test]
    fn inverse_is_two_sided_inverse_for_non_unit_quaternion() {
        let q = with_components(1.0, 2.0, 3.0, 4.0);
        let inv = q.inverse().unwrap();
        let identity = with_components(0.0, 0.0, 0.0, 1.0);
        assert!(approx_quat(q * inv, identity));
        assert!(approx_quat(inv * q, identity));
    }

    #[test]
    fn inverse_of_zero_quaternion_errs() {
        let q = with_components(0.0, 0.0, 0.0, 0.0);
        assert_eq!(q.inverse(), Err(ZeroLengthQuaternion));
    }

    #[test]
    fn mul_composes_two_quarter_turns_into_a_half_turn() {
        let q90 = Quaternion::<f64>::from_euler(0.0, 0.0, PI / 2.0);
        let q180 = Quaternion::<f64>::from_euler(0.0, 0.0, PI);
        assert!(approx_quat(q90 * q90, q180));
    }

    #[test]
    fn mul_identity_is_identity_element() {
        let identity = Quaternion::<f64>::from_euler(0.0, 0.0, 0.0);
        let q = Quaternion::<f64>::from_euler(0.3, 0.5, 0.7);
        assert!(approx_quat(identity * q, q));
        assert!(approx_quat(q * identity, q));
    }
}

use crate::mathematics::traits::Number;
use core::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Angle<T: Number>(T);

impl<T: Number> Angle<T> {
    pub fn from_radians(angle: T) -> Self {
        Self(angle)
    }

    pub fn from_degrees(angle: T) -> Self {
        Self(angle * T::pi() / T::from(180.0f32))
    }

    pub fn to_radians(&self) -> T {
        self.0
    }

    pub fn to_degrees(&self) -> T {
        self.0 * T::from(180.0f32) / T::pi()
    }
}

/// Add one angle to another.
impl<T: Number> Add<Angle<T>> for Angle<T> {
    type Output = Self;
    fn add(self, rhs: Angle<T>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Number> AddAssign<Angle<T>> for Angle<T> {
    fn add_assign(&mut self, rhs: Angle<T>) {
        self.0 += rhs.0;
    }
}

/// Subtract one angle from another.
impl<T: Number> Sub<Angle<T>> for Angle<T> {
    type Output = Self;
    fn sub(self, rhs: Angle<T>) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<T: Number> SubAssign<Angle<T>> for Angle<T> {
    fn sub_assign(&mut self, rhs: Angle<T>) {
        self.0 -= rhs.0;
    }
}

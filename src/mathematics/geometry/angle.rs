use std::f32::consts::PI;

use crate::mathematics::traits::Number;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Angle<T: Number>(T);

impl<T: Number> Angle<T> {
    pub fn from_radians(angle: T) -> Self {
        Self(angle)
    }

    pub fn from_degrees(angle: T) -> Self {
        Self(angle * T::pi()/180.0)
    }


}


use std::ops::{Add, Div, Mul, Sub};

pub trait NumOps:
    Sized
    + Copy
    + Clone
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialEq
{
    fn sqrt(&self) -> Self;
    fn zero() -> Self;
}

impl NumOps for f32 {
    fn sqrt(&self) -> Self {
        f32::sqrt(*self)
    }

    fn zero() -> Self {
        0f32
    }
}

impl NumOps for f64 {
    fn sqrt(&self) -> Self {
        f64::sqrt(*self)
    }

    fn zero() -> Self {
        0f64
    }
}

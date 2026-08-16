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
    + PartialOrd
{
    fn sqrt(&self) -> Self;
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
}

impl NumOps for f32 {
    fn sqrt(&self) -> Self {
        f32::sqrt(*self)
    }

    fn sin(&self) -> Self {
        f32::sin(*self)
    }

    fn cos(&self) -> Self {
        f32::cos(*self)
    }

    fn zero() -> Self {
        0f32
    }

    fn one() -> Self {
        1f32
    }
}

impl NumOps for f64 {
    fn sqrt(&self) -> Self {
        f64::sqrt(*self)
    }

    fn sin(&self) -> Self {
        f64::sin(*self)
    }

    fn cos(&self) -> Self {
        f64::cos(*self)
    }

    fn zero() -> Self {
        0f64
    }

    fn one() -> Self {
        1f64
    }
}

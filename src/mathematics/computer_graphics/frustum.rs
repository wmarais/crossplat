use crate::mathematics::traits::Number;
use crate::mathematics::linear_algebra::Plane;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Frustum<T: Number, const N: usize>([Plane<T, N>; 6]);

impl<T: Number, const N: usize> Frustum<T, N> {
    const IDX_NEAR: usize = 0;
    const IDX_FAR: usize = 1;
    const IDX_LEFT: usize = 2;
    const IDX_RIGHT: usize = 3;
    const IDX_TOP: usize = 4;
    const IDX_BOTTOM: usize = 5;

    pub fn near(&self) -> &Plane<T, N> {
        &self.0[Self::IDX_NEAR]
    }

    pub fn far(&self) -> &Plane<T, N> {
        &self.0[Self::IDX_FAR]
    }

    pub fn left(&self) -> &Plane<T, N> {
        &self.0[Self::IDX_LEFT]
    }

    pub fn right(&self) -> &Plane<T, N> {
        &self.0[Self::IDX_RIGHT]
    }

    pub fn top(&self) -> &Plane<T, N> {
        &self.0[Self::IDX_TOP]
    }

    pub fn bottom(&self) -> &Plane<T, N> {
        &self.0[Self::IDX_BOTTOM]
    }

}
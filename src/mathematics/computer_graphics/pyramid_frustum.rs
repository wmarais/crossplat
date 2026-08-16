use crate::mathematics::geometry::{Circle, Plane, Ray, Rectangle};
use crate::mathematics::traits::Number;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PyramidFrustum<T: Number, const N: usize>([Plane<T, N>; 6]);

impl<T: Number, const N: usize> PyramidFrustum<T, N> {
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

    // /// Determine if the ray intersect the frustum or not.
    // pub fn intersect_ray(&self, ray: &Ray<T,N>) -> bool {

    // }

    // /// Determine if the ray intersects the frustum or not.
    // pub fn intersect_rectangle(&self, rect: &Rectangle<T, N>) -> bool {

    // }

    // pub fn intersect_circle(&self, circle: &Circle<T, N>) -> bool {

    // }
}

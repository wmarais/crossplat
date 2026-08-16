use crate::mathematics::traits::Number;
use crate::mathematics::linear_algebra::{Ray, Vector};

/// The plane definition is packed as a tuple, where the first element of the tuple is
/// the normal of the plane, and the second element of the tuple is the distance from
/// the origin.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Plane<T: Number, const K: usize>((Vector<T, K>, T));

impl<T: Number, const K: usize> Plane<T, K> {
    /// Calculate where a ray intersects the plane. The function returns either:
    /// `Some(T)` if there is a single point of intersection. If the value is negative, it
    /// is behind the start of the ray, and if the value is positive, it is in front of the
    /// start of the ray.
    /// `None` if the ray is co-planar (lies flat on the plane) or does not intersect the
    /// plane at all (i.e. it is parallel to the plane).
    pub fn intersect_ray(&self, ray: &Ray<T, K>) -> Option<T> {
        let (normal, distance) = self.0;
        let denom = normal.dot(&ray.direction());

        if denom.approx_equal_small(&T::zero()) {
            return None;
        }

        let origin = Vector::from(ray.origin());
        Some((distance - normal.dot(&origin)) / denom)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Point;
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn ray_hits_plane_head_on() {
        let plane = Plane((Vector::new([0.0, 0.0, 1.0]), 0.0));
        let ray = Ray::new(Point::new([0.0, 0.0, -5.0]), Vector::new([0.0, 0.0, 1.0]));
        assert!(approx(plane.intersect_ray(&ray).unwrap(), 5.0));
    }

    #[test]
    fn ray_parallel_to_plane_returns_none() {
        let plane = Plane((Vector::new([0.0, 0.0, 1.0]), 0.0));
        let ray = Ray::new(Point::new([0.0, 0.0, 5.0]), Vector::new([1.0, 0.0, 0.0]));
        assert_eq!(plane.intersect_ray(&ray), None);
    }

    #[test]
    fn ray_pointing_away_hits_behind_origin() {
        let plane = Plane((Vector::new([0.0, 0.0, 1.0]), 0.0));
        let ray = Ray::new(Point::new([0.0, 0.0, 5.0]), Vector::new([0.0, 0.0, 1.0]));
        assert!(approx(plane.intersect_ray(&ray).unwrap(), -5.0));
    }

    #[test]
    fn ray_hits_offset_plane_at_an_angle() {
        let plane = Plane((Vector::new([1.0, 0.0, 0.0]), 5.0));
        let ray = Ray::new(Point::new([0.0, 0.0, 0.0]), Vector::new([1.0, 1.0, 0.0]));
        assert!(approx(plane.intersect_ray(&ray).unwrap(), 5.0));
    }
}

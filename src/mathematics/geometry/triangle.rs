use crate::mathematics::traits::Number;
use crate::mathematics::geometry::{Point, Ray};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Triangle<T: Number, const N: usize>([Point<T, N>; 3]);

impl<T: Number> Triangle<T, 3> {
    /// Calculate where a ray intersects the triangle, using the Moller-Trumbore algorithm.
    /// The function returns either:
    /// `Some(T)` giving the ray parameter at the point of intersection. If the value is
    /// negative, it is behind the start of the ray, and if the value is positive, it is in
    /// front of the start of the ray.
    /// `None` if the ray is parallel to the triangle's plane, or if it crosses the plane
    /// outside of the triangle's bounds.
    pub fn intersect_ray(&self, ray: &Ray<T, 3>) -> Option<T> {
        let [v0, v1, v2] = self.0;

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;

        let h = ray.direction().cross(&edge2);
        let det = edge1.dot(&h);

        if det.approx_equal_small(&T::zero()) {
            return None;
        }

        let f = T::one() / det;
        let s = ray.origin() - v0;
        let u = f * s.dot(&h);

        if u < T::zero() || u > T::one() {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * ray.direction().dot(&q);

        if v < T::zero() || (u + v) > T::one() {
            return None;
        }

        Some(f * edge2.dot(&q))
    }
}

#[cfg(test)]
mod tests {
    use crate::mathematics::linear_algebra::Vector;
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn unit_triangle() -> Triangle<f64, 3> {
        Triangle([
            Point::new([0.0, 0.0, 0.0]),
            Point::new([1.0, 0.0, 0.0]),
            Point::new([0.0, 1.0, 0.0]),
        ])
    }

    #[test]
    fn ray_through_centre_hits_triangle() {
        let triangle = unit_triangle();
        let ray = Ray::new(Point::new([0.2, 0.2, -5.0]), Vector::new([0.0, 0.0, 1.0]));
        assert!(approx(triangle.intersect_ray(&ray).unwrap(), 5.0));
    }

    #[test]
    fn ray_missing_triangle_returns_none() {
        let triangle = unit_triangle();
        let ray = Ray::new(Point::new([5.0, 5.0, -5.0]), Vector::new([0.0, 0.0, 1.0]));
        assert_eq!(triangle.intersect_ray(&ray), None);
    }

    #[test]
    fn ray_parallel_to_triangle_returns_none() {
        let triangle = unit_triangle();
        let ray = Ray::new(Point::new([0.2, 0.2, 1.0]), Vector::new([1.0, 0.0, 0.0]));
        assert_eq!(triangle.intersect_ray(&ray), None);
    }

    #[test]
    fn ray_pointing_away_hits_behind_origin() {
        let triangle = unit_triangle();
        let ray = Ray::new(Point::new([0.2, 0.2, 5.0]), Vector::new([0.0, 0.0, 1.0]));
        assert!(approx(triangle.intersect_ray(&ray).unwrap(), -5.0));
    }
}

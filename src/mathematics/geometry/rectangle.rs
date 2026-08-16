use crate::mathematics::geometry::{Point, Ray};
use crate::mathematics::traits::Number;

/// An axis-aligned N-Dimensional rectangle (bounding box), packed as an array where the
/// first element is the minimum corner and the second element is the maximum corner.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rectangle<T: Number, const N: usize>([Point<T, N>; 2]);

impl<T: Number, const N: usize> Rectangle<T, N> {
    /// Calculate the intersection of a Ray with the rectangle, using the slab method. The
    /// function returns either:
    /// `(None, None)` indicating that the ray does not intersect the rectangle.
    /// `(T1, T2)` indicating that the ray enters the rectangle at T1 and exits at T2 (T1 is
    /// always less than or equal to T2). If a value is negative, it is behind the start of
    /// the ray, and if the value is positive, it is in front of the start of the ray.
    pub fn intersect_ray(&self, ray: &Ray<T, N>) -> (Option<T>, Option<T>) {
        let min = self.0[0];
        let max = self.0[1];
        let origin = ray.origin();
        let dir = ray.direction();

        let (mut t_near, mut t_far) = Self::axis_interval(min[0], max[0], origin[0], dir[0]);

        for k in 1..N {
            let (axis_near, axis_far) = Self::axis_interval(min[k], max[k], origin[k], dir[k]);

            if axis_near > t_near {
                t_near = axis_near;
            }

            if axis_far < t_far {
                t_far = axis_far;
            }
        }

        if t_near > t_far {
            (None, None)
        } else {
            (Some(t_near), Some(t_far))
        }
    }

    /// Compute the ray parameters at which it crosses the two slab planes of a single axis,
    /// ordered so the first is the nearer crossing. A zero direction component naturally
    /// yields +/- infinity via IEEE-754 division, which leaves that axis unconstrained
    /// whenever the origin already lies within the slab.
    fn axis_interval(min: T, max: T, origin: T, dir: T) -> (T, T) {
        let t1 = (min - origin) / dir;
        let t2 = (max - origin) / dir;

        if t1 <= t2 {
            (t1, t2)
        } else {
            (t2, t1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mathematics::linear_algebra::Vector;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn unit_cube() -> Rectangle<f64, 3> {
        Rectangle([Point::new([0.0, 0.0, 0.0]), Point::new([1.0, 1.0, 1.0])])
    }

    #[test]
    fn ray_enters_and_exits_the_box() {
        let cube = unit_cube();
        let ray = Ray::new(Point::new([-5.0, 0.5, 0.5]), Vector::new([1.0, 0.0, 0.0]));
        let (t1, t2) = cube.intersect_ray(&ray);
        assert!(approx(t1.unwrap(), 5.0));
        assert!(approx(t2.unwrap(), 6.0));
    }

    #[test]
    fn ray_missing_the_box_returns_none() {
        let cube = unit_cube();
        let ray = Ray::new(Point::new([-5.0, 5.0, 5.0]), Vector::new([1.0, 0.0, 0.0]));
        assert_eq!(cube.intersect_ray(&ray), (None, None));
    }

    #[test]
    fn ray_starting_inside_the_box_has_negative_and_positive_hit() {
        let cube = unit_cube();
        let ray = Ray::new(Point::new([0.5, 0.5, 0.5]), Vector::new([1.0, 0.0, 0.0]));
        let (t1, t2) = cube.intersect_ray(&ray);
        assert!(approx(t1.unwrap(), -0.5));
        assert!(approx(t2.unwrap(), 0.5));
    }

    #[test]
    fn ray_parallel_to_an_axis_within_the_slab_still_hits() {
        let cube = unit_cube();
        let ray = Ray::new(Point::new([0.5, -5.0, 0.5]), Vector::new([0.0, 1.0, 0.0]));
        let (t1, t2) = cube.intersect_ray(&ray);
        assert!(approx(t1.unwrap(), 5.0));
        assert!(approx(t2.unwrap(), 6.0));
    }

    #[test]
    fn ray_parallel_to_an_axis_outside_the_slab_misses() {
        let cube = unit_cube();
        let ray = Ray::new(Point::new([5.0, -5.0, 0.5]), Vector::new([0.0, 1.0, 0.0]));
        assert_eq!(cube.intersect_ray(&ray), (None, None));
    }
}

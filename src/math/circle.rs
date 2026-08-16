use super::NumOps;
use super::{Point, Ray};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Circle<T: NumOps, const K: usize> {
    centre: Point<T, K>,
    radius: T,
}

impl<T: NumOps, const K: usize> Circle<T, K> {
    /// Collision detection function between a ray and a circle.
    /// Calculate the intersection of a N-Dimensional Ray with an N-Dimensional circle. The
    /// function returns either:
    /// `(None, None)` indicating that the ray does not intersect the circle.
    /// `(T1, None)` indicating that ray is tangential to the circle (i.e. on contact point).
    /// `(T1, T2)` indicating that the ray passess through the circle. T1 is the first intersection
    /// and T2 is the second intersection.
    /// If the value (either T1 or T2) are negative, it is behind the start of the ray, and if the
    /// value is positive, it is in front of the start of the ray.
    pub fn intersect_ray(&self, ray: &Ray<T, K>) -> (Option<T>, Option<T>) {
        let oc = ray.origin() - self.centre;
        let dir = ray.direction();

        let a = dir.dot(&dir);
        let half_b = oc.dot(&dir);
        let c = oc.dot(&oc) - self.radius * self.radius;

        // Quarter of the full discriminant (the full b is 2 * half_b), which lets the
        // roots below be expressed without needing a literal `2` or `4` from T.
        let quarter_disc = half_b * half_b - a * c;

        if quarter_disc < T::zero() {
            return (None, None);
        }

        let sqrt_qd = quarter_disc.sqrt();
        let t1 = (T::zero() - half_b - sqrt_qd) / a;

        if quarter_disc == T::zero() {
            (Some(t1), None)
        } else {
            let t2 = (T::zero() - half_b + sqrt_qd) / a;
            (Some(t1), Some(t2))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Vector;
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn ray_through_centre_hits_twice() {
        let circle = Circle {
            centre: Point::new([0.0, 0.0]),
            radius: 2.0,
        };
        let ray = Ray::new(Point::new([-5.0, 0.0]), Vector::new([1.0, 0.0]));
        let (t1, t2) = circle.intersect_ray(&ray);
        assert!(approx(t1.unwrap(), 3.0));
        assert!(approx(t2.unwrap(), 7.0));
    }

    #[test]
    fn ray_missing_circle_returns_none() {
        let circle = Circle {
            centre: Point::new([0.0, 0.0]),
            radius: 1.0,
        };
        let ray = Ray::new(Point::new([-5.0, 5.0]), Vector::new([1.0, 0.0]));
        assert_eq!(circle.intersect_ray(&ray), (None, None));
    }

    #[test]
    fn tangent_ray_hits_once() {
        let circle = Circle {
            centre: Point::new([0.0, 0.0]),
            radius: 1.0,
        };
        let ray = Ray::new(Point::new([-5.0, 1.0]), Vector::new([1.0, 0.0]));
        let (t1, t2) = circle.intersect_ray(&ray);
        assert!(approx(t1.unwrap(), 5.0));
        assert_eq!(t2, None);
    }

    #[test]
    fn ray_starting_inside_circle_has_negative_and_positive_hit() {
        let circle = Circle {
            centre: Point::new([0.0, 0.0]),
            radius: 3.0,
        };
        let ray = Ray::new(Point::new([0.0, 0.0]), Vector::new([1.0, 0.0]));
        let (t1, t2) = circle.intersect_ray(&ray);
        assert!(approx(t1.unwrap(), -3.0));
        assert!(approx(t2.unwrap(), 3.0));
    }
}

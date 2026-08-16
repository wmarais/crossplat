pub trait ApproxEqual {
    /// Implement an approximate equality function for comparing a mixture
    /// of small and large numbers. Generally used where it is not clear whether
    /// the compared numbers will be large or small.
    fn approx_equal(&self, rhs: &Self) -> bool;

    /// Used to compare small numbers with each other. This is generally
    /// used where the size of values are confined, i.e. unit vectors,
    /// or other normalised values.
    fn approx_equal_small(&self, rhs: &Self) -> bool;

    /// Used to compare large numbers with each other. This is generally
    /// used where the size of the values are generally expected to be
    /// large, i.e. astronomical distances etc.
    fn approx_equal_large(&self, rhs: &Self) -> bool;
}

impl ApproxEqual for f32 {
    fn approx_equal(&self, rhs: &Self) -> bool {
        self.approx_equal_small(rhs) || self.approx_equal_large(rhs)
    }

    fn approx_equal_small(&self, rhs: &Self) -> bool {
        (self - rhs).abs() <= Self::EPSILON
    }

    fn approx_equal_large(&self, rhs: &Self) -> bool {
        let largest = self.abs().max(rhs.abs());
        (self - rhs).abs() <= largest * Self::EPSILON
    }
}

impl ApproxEqual for f64 {
    fn approx_equal(&self, rhs: &Self) -> bool {
        self.approx_equal_small(rhs) || self.approx_equal_large(rhs)
    }

    fn approx_equal_small(&self, rhs: &Self) -> bool {
        (self - rhs).abs() <= Self::EPSILON
    }

    fn approx_equal_large(&self, rhs: &Self) -> bool {
        let largest = self.abs().max(rhs.abs());
        (self - rhs).abs() <= largest * Self::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_small_values_within_epsilon_are_equal() {
        assert!(1.0f32.approx_equal_small(&(1.0 + f32::EPSILON)));
        assert!(0.0f32.approx_equal_small(&0.0));
    }

    #[test]
    fn f32_small_values_outside_epsilon_are_not_equal() {
        assert!(!1.0f32.approx_equal_small(&1.001));
    }

    #[test]
    fn f32_large_values_within_relative_epsilon_are_equal() {
        let a = 1.0e9f32;
        let b = a + a * f32::EPSILON * 0.1;
        assert!(a.approx_equal_large(&b));
    }

    #[test]
    fn f32_large_values_outside_relative_epsilon_are_not_equal() {
        let a = 1.0e9f32;
        let b = a + a * f32::EPSILON * 100.0;
        assert!(!a.approx_equal_large(&b));
    }

    #[test]
    fn f32_approx_equal_covers_both_small_and_large_magnitudes() {
        let a = 1.0e9f32;
        let b = a + a * f32::EPSILON * 0.1;
        assert!(0.0f32.approx_equal(&0.0));
        assert!(a.approx_equal(&b));
        assert!(!1.0f32.approx_equal(&1.001));
    }

    #[test]
    fn f64_small_values_within_epsilon_are_equal() {
        assert!(1.0f64.approx_equal_small(&(1.0 + f64::EPSILON)));
        assert!(0.0f64.approx_equal_small(&0.0));
    }

    #[test]
    fn f64_small_values_outside_epsilon_are_not_equal() {
        assert!(!1.0f64.approx_equal_small(&1.001));
    }

    #[test]
    fn f64_large_values_within_relative_epsilon_are_equal() {
        let a = 1.0e15f64;
        let b = a + a * f64::EPSILON * 0.1;
        assert!(a.approx_equal_large(&b));
    }

    #[test]
    fn f64_large_values_outside_relative_epsilon_are_not_equal() {
        let a = 1.0e15f64;
        let b = a + a * f64::EPSILON * 100.0;
        assert!(!a.approx_equal_large(&b));
    }

    #[test]
    fn f64_approx_equal_covers_both_small_and_large_magnitudes() {
        let a = 1.0e15f64;
        let b = a + a * f64::EPSILON * 0.1;
        assert!(0.0f64.approx_equal(&0.0));
        assert!(a.approx_equal(&b));
        assert!(!1.0f64.approx_equal(&1.001));
    }
}

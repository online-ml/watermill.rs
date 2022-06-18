use crate::traits::Univariate;
use num::{Float, FromPrimitive};
use std::ops::{AddAssign, SubAssign};
/// Running min.
/// # Examples
/// ```
/// use online_statistics::minimum::Min;
/// use online_statistics::traits::Univariate;
/// let mut running_min: Min<f64> = Min::new();
/// for i in 1..10{
///     running_min.update(i as f64);
/// }
/// assert_eq!(running_min.get(), 1.0);
/// ```
///
#[derive(Clone, Copy, Default, Debug)]
pub struct Min<F: Float + FromPrimitive + AddAssign + SubAssign> {
    pub min: F,
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> Min<F> {
    pub fn new() -> Self {
        Self {
            min: F::max_value(),
        }
    }
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> Univariate<F> for Min<F> {
    fn update(&mut self, x: F) {
        if self.min > x {
            self.min = x;
        }
    }
    fn get(self) -> F {
        self.min
    }
}

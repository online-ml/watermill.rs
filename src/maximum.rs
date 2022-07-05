use crate::traits::Univariate;
use num::{Float, FromPrimitive};
use std::ops::{AddAssign, SubAssign};

/// Running max.
/// # Examples
/// ```
/// use online_statistics::maximum::Max;
/// use online_statistics::traits::Univariate;
/// let mut running_max: Max<f64> = Max::new();
/// for i in 1..10{
///     running_max.update(i as f64);
/// }
/// assert_eq!(running_max.get(), 9.0);
/// ```
///
#[derive(Clone, Copy, Default, Debug)]
pub struct Max<F: Float + FromPrimitive + AddAssign + SubAssign> {
    pub max: F,
}
impl<F: Float + FromPrimitive + AddAssign + SubAssign> Max<F> {
    pub fn new() -> Self {
        Self {
            max: F::min_value(),
        }
    }
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> Univariate<F> for Max<F> {
    fn update(&mut self, x: F) {
        if self.max < x {
            self.max = x;
        }
    }
    fn get(&mut self) -> F {
        self.max
    }
}

/// Running absolute max.
/// # Examples
/// ```
/// use online_statistics::maximum::AbsMax;
/// use online_statistics::traits::Univariate;
/// let mut running_abs_max: AbsMax<f64> = AbsMax::new();
/// for i in -17..10{
///     running_abs_max.update(i as f64);
/// }
/// assert_eq!(running_abs_max.get(), 17.0);
/// ```
///
#[derive(Default, Debug)]
pub struct AbsMax<F: Float + FromPrimitive + AddAssign + SubAssign> {
    abs_max: F,
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> AbsMax<F> {
    pub fn new() -> Self {
        Self {
            abs_max: F::from_f64(0.0).unwrap(),
        }
    }
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> Univariate<F> for AbsMax<F> {
    fn update(&mut self, x: F) {
        if self.abs_max < x.abs() {
            self.abs_max = x.abs();
        }
    }
    fn get(&mut self) -> F {
        self.abs_max
    }
}

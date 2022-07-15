use crate::sorted_window::SortedWindow;
use crate::traits::Univariate;
use num::{Float, FromPrimitive};
use std::ops::{AddAssign, SubAssign};
/// Running min.
/// # Arguments
/// * `window_size` - Size of the rolling window.
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
    fn get(&mut self) -> F {
        self.min
    }
}

/// Rolling min.
/// # Examples
/// ```
/// use online_statistics::minimum::RollingMin;
/// use online_statistics::traits::Univariate;
/// let mut rolling_min: RollingMin<f64> = RollingMin::new(3);
/// for i in 1..10{
///     rolling_min.update(i as f64);
/// }
/// assert_eq!(rolling_min.get(), 7.0);
/// ```
///
pub struct RollingMin<F: Float + FromPrimitive + AddAssign + SubAssign> {
    sorted_window: SortedWindow<F>,
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> RollingMin<F> {
    pub fn new(window_size: usize) -> Self {
        Self {
            sorted_window: SortedWindow::new(window_size),
        }
    }
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> Univariate<F> for RollingMin<F> {
    fn update(&mut self, x: F) {
        self.sorted_window.push_back(x);
    }
    fn get(&mut self) -> F {
        self.sorted_window.front()
    }
}

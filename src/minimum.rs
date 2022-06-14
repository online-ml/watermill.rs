use crate::traits::OnlineStatistic;
use num::{Float, FromPrimitive};

/// Running min.
/// # Examples
/// ```
/// use online_statistics::minimum::Min;
/// use online_statistics::traits::OnlineStatistic;
/// let mut running_min: Min<f64> = Min::new();
/// for i in 1..10{
///     running_min.update(i as f64);
/// }
/// assert_eq!(running_min.get(), 1.0);
/// ```
///
#[derive(Default, Debug)]
pub struct Min<F: Float + FromPrimitive> {
    pub min: F,
}

impl<F: Float + FromPrimitive> Min<F> {
    pub fn new() -> Self {
        Self {
            min: F::max_value(),
        }
    }
}

impl<F: Float + FromPrimitive> OnlineStatistic<F> for Min<F> {
    fn update(&mut self, x: F) {
        if self.min > x {
            self.min = x;
        }
    }
    fn get(self) -> F {
        self.min
    }
}

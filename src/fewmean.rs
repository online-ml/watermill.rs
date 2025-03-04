use num::{Float, FromPrimitive};
use std::ops::{AddAssign, SubAssign};
use crate::stats::Univariate;

/// **Fading Exponentially Weighted Mean (FEWMean)**
///
/// This structure computes an **incrementally updated exponentially weighted mean** with a fading factor.
///
/// ## Difference from `ewmean.rs`:
/// - `EWMean` is a classic **exponential moving average (EMA)** that applies a fixed smoothing factor (`alpha`).
/// - The first few updates in `FEWMean` match the exact mean of a finite window of past values, whereas `EWMean` converges towards it **asymptotically**.
/// - This is achieved by keeping track of a **decaying weight sum**, ensuring a more precise adaptation in early updates.
///
/// # Arguments
/// * `fading_factor` - Controls the decay of past values. Smaller values give more weight to past data.
///
/// # Examples
/// ```
/// use watermill::fewmean::FEWMean;
/// use watermill::stats::Univariate;
///
/// let mut few_mean: FEWMean<f64> = FEWMean::default();
/// let data = vec![1., 3., 5., 4., 6., 8., 7., 9., 11.];
/// for &x in data.iter() {
///     few_mean.update(x);
/// }
/// assert_eq!(few_mean.get(), 9.4296875);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct FEWMean<F: Float + FromPrimitive + AddAssign + SubAssign> {
    mean: F,
    fading_factor: F,
    weight_sum: F,
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> FEWMean<F> {
    /// Creates a new `FEWMean` instance with a given fading factor.
    pub fn new(fading_factor: F) -> Self {
        Self {
            mean: F::from_f64(0.0).unwrap(),
            fading_factor,
            weight_sum: F::from_f64(0.0).unwrap(),
        }
    }
}

impl<F> Default for FEWMean<F>
where
    F: Float + FromPrimitive + AddAssign + SubAssign,
{
    fn default() -> Self {
        Self::new(F::from_f64(0.01).unwrap())
    }
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> Univariate<F> for FEWMean<F> {
    fn update(&mut self, x: F) {
        if self.weight_sum == F::from_f64(0.0).unwrap() {
            self.mean = x;
            self.weight_sum = F::from_f64(1.0).unwrap();
        } else {
            let weight = (F::from_f64(1.0).unwrap() - self.fading_factor) * self.weight_sum;
            self.mean = (weight * self.mean + x) / (weight + F::from_f64(1.0).unwrap());
            self.weight_sum = weight + F::from_f64(1.0).unwrap();
        }
    }

    fn get(&self) -> F {
        self.mean
    }
}

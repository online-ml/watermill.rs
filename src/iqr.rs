use crate::quantile::Quantile;

use crate::stats::Univariate;
use num::{Float, FromPrimitive};
use serde::{Deserialize, Serialize};
use std::ops::{AddAssign, SubAssign};
/// Computes the interquartile range.
/// # Arguments
/// * `q_inf` - Desired inferior quantile, must be between 0 and 1. Defaults to `0.25`.
/// * `q_sup` -  Desired superior quantile, must be between 0 and 1. Defaults to `0.75`.
/// # Examples
/// ```
/// use online_statistics::iqr::IQR;
/// use online_statistics::stats::Univariate;
/// let mut running_iqr: IQR<f64> = IQR::default();
/// for i in 1..=100{
///     running_iqr.update(i as f64);
/// }
/// assert_eq!(running_iqr.get(), 50.0);
/// ```
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IQR<F: Float + FromPrimitive + AddAssign + SubAssign> {
    pub q_inf: Quantile<F>,
    pub q_sup: Quantile<F>,
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> IQR<F> {
    pub fn new(q_inf: F, q_sup: F) -> Result<Self, &'static str> {
        if q_inf >= q_sup {
            return Err("q_inf must be strictly less than q_sup");
        }

        Ok(Self {
            q_inf: Quantile::new(q_inf).unwrap(),
            q_sup: Quantile::new(q_sup).unwrap(),
        })
    }
}

impl<F> Default for IQR<F>
where
    F: Float + FromPrimitive + AddAssign + SubAssign,
{
    fn default() -> Self {
        Self {
            q_inf: Quantile::new(F::from_f64(0.25).unwrap()).unwrap(),
            q_sup: Quantile::new(F::from_f64(0.75).unwrap()).unwrap(),
        }
    }
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> Univariate<F> for IQR<F> {
    fn update(&mut self, x: F) {
        self.q_inf.update(x);
        self.q_sup.update(x);
    }
    fn get(&self) -> F {
        self.q_sup.get() - self.q_inf.get()
    }
}

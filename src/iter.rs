use num::{Float, FromPrimitive};
use std::ops::{AddAssign, SubAssign};

use crate::count::Count;
use crate::ewmean::EWMean;
use crate::ewvariance::EWVariance;
use crate::iqr::IQR;
use crate::kurtosis::Kurtosis;
use crate::maximum::{AbsMax, Max};
use crate::mean::Mean;
use crate::minimum::Min;
use crate::ptp::PeakToPeak;
use crate::quantile::Quantile;
use crate::skew::Skew;
use crate::sum::Sum;
use crate::traits::Univariate;
use crate::variance::Variance;
pub struct IterStat<I>
where
    I: Iterator,
    I::Item: Float + FromPrimitive + AddAssign + SubAssign + 'static,
{
    stat: Box<dyn Univariate<I::Item>>,
    underlying: I,
}

impl<I> Iterator for IterStat<I>
where
    I: Iterator,
    I::Item: Float + FromPrimitive + AddAssign + SubAssign,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.underlying.next() {
            self.stat.update(x);
            return Some(self.stat.get());
        }
        None
    }
}

pub trait IterStatExt: Iterator {
    fn online_sum(self) -> IterStat<Self>
    where
        Self::Item: Float + FromPrimitive + AddAssign + SubAssign,
        Self: Sized,
    {
        IterStat {
            stat: Box::new(Sum::new()),
            underlying: self,
        }
    }

    fn online_mean(self) -> IterStat<Self>
    where
        Self::Item: Float + FromPrimitive + AddAssign + SubAssign,
        Self: Sized,
    {
        IterStat {
            stat: Box::new(Mean::new()),
            underlying: self,
        }
    }
    fn online_count(self) -> IterStat<Self>
    where
        Self::Item: Float + FromPrimitive + AddAssign + SubAssign,
        Self: Sized,
    {
        IterStat {
            stat: Box::new(Count::new()),
            underlying: self,
        }
    }

    fn online_ewmean(self, alpha: Self::Item) -> IterStat<Self>
    where
        Self::Item: Float + FromPrimitive + AddAssign + SubAssign,
        Self: Sized,
    {
        IterStat {
            stat: Box::new(EWMean::new(alpha)),
            underlying: self,
        }
    }

    fn online_ewvar(self, alpha: Self::Item) -> IterStat<Self>
    where
        Self::Item: Float + FromPrimitive + AddAssign + SubAssign,
        Self: Sized,
    {
        IterStat {
            stat: Box::new(EWVariance::new(alpha)),
            underlying: self,
        }
    }

    fn online_iqr(self, q_inf: Self::Item, q_sup: Self::Item) -> IterStat<Self>
    where
        Self::Item: Float + FromPrimitive + AddAssign + SubAssign,
        Self: Sized,
    {
        IterStat {
            stat: Box::new(IQR::new(q_inf, q_sup).expect("q_inf must be strictly less than q_sup")),
            underlying: self,
        }
    }
    fn online_kurtosis(self, bias: bool) -> IterStat<Self>
    where
        Self::Item: Float + FromPrimitive + AddAssign + SubAssign,
        Self: Sized,
    {
        IterStat {
            stat: Box::new(Kurtosis::new(bias)),
            underlying: self,
        }
    }
    fn online_max(self) -> IterStat<Self>
    where
        Self::Item: Float + FromPrimitive + AddAssign + SubAssign,
        Self: Sized,
    {
        IterStat {
            stat: Box::new(Max::new()),
            underlying: self,
        }
    }

    fn online_abs_max(self) -> IterStat<Self>
    where
        Self::Item: Float + FromPrimitive + AddAssign + SubAssign,
        Self: Sized,
    {
        IterStat {
            stat: Box::new(AbsMax::new()),
            underlying: self,
        }
    }

    fn online_min(self) -> IterStat<Self>
    where
        Self::Item: Float + FromPrimitive + AddAssign + SubAssign,
        Self: Sized,
    {
        IterStat {
            stat: Box::new(Min::new()),
            underlying: self,
        }
    }
    fn online_ptp(self) -> IterStat<Self>
    where
        Self::Item: Float + FromPrimitive + AddAssign + SubAssign,
        Self: Sized,
    {
        IterStat {
            stat: Box::new(PeakToPeak::new()),
            underlying: self,
        }
    }
    fn online_quantile(self, q: Self::Item) -> IterStat<Self>
    where
        Self::Item: Float + FromPrimitive + AddAssign + SubAssign,
        Self: Sized,
    {
        IterStat {
            stat: Box::new(Quantile::new(q).expect("q should be betweek 0 and 1")),
            underlying: self,
        }
    }
    fn online_skew(self, bias: bool) -> IterStat<Self>
    where
        Self::Item: Float + FromPrimitive + AddAssign + SubAssign,
        Self: Sized,
    {
        IterStat {
            stat: Box::new(Skew::new(bias)),
            underlying: self,
        }
    }
    fn online_var(self, ddof: u32) -> IterStat<Self>
    where
        Self::Item: Float + FromPrimitive + AddAssign + SubAssign,
        Self: Sized,
    {
        IterStat {
            stat: Box::new(Variance::new(ddof)),
            underlying: self,
        }
    }
}
impl<I: Iterator> IterStatExt for I {}

#[cfg(test)]
mod tests {

    use super::IterStatExt;

    #[test]
    fn test_online_sum() {
        let data: Vec<f64> = vec![1., 2., 3.];
        let vec_true: Vec<f64> = vec![1., 3., 6.];
        for (d, t) in data.into_iter().online_sum().zip(vec_true.into_iter()) {
            assert_eq!(d, t);
        }
    }
}

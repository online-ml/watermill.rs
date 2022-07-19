use num::{Float, FromPrimitive};
use std::ops::{AddAssign, SubAssign};
pub trait Univariate<F: Float + FromPrimitive + AddAssign + SubAssign> {
    fn update(&mut self, x: F);
    ///The use of `get` as a `&mut self` method is highly debatable, the only use is for computing
    /// online quantiles. I haven't found a satisfying solution to make `get` non-mutable.
    fn get(&self) -> F;
}

pub trait Bivariate<F: Float + FromPrimitive + AddAssign + SubAssign> {
    fn update(&mut self, x: F, y: F);
    fn get(&self) -> F;
}

pub trait Rollable<F: Float + FromPrimitive + AddAssign + SubAssign> {
    fn revert(&mut self, x: F) -> Result<(), &'static str>;
}

pub trait RollableUnivariate<F: Float + FromPrimitive + AddAssign + SubAssign>:
    Rollable<F> + Univariate<F>
{
}

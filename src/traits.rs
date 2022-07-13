use num::{Float, FromPrimitive};
use std::ops::{AddAssign, SubAssign};
pub trait Univariate<F: Float + FromPrimitive + AddAssign + SubAssign> {
    fn update(&mut self, x: F);
    ///The use of `get` as a `&mut self` method is highly debatable, the only use is for computing
    /// online quantiles. I haven't found a satisfying solution to make `get` non-mutable.
    fn get(&mut self) -> F;
}

pub trait Bivariate<F: Float + FromPrimitive + AddAssign + SubAssign> {
    fn update(&mut self, x: F, y: F);
    fn get(&mut self) -> F;
}

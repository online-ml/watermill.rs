use num::{Float, FromPrimitive};
use std::ops::{AddAssign, SubAssign};
pub trait Univariate<F: Float + FromPrimitive + AddAssign + SubAssign> {
    fn update(&mut self, x: F);
    fn get(self) -> F;
}

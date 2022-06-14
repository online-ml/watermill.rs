use num::{Float, FromPrimitive};
use std::ops::AddAssign;
pub trait OnlineStatistic<F: Float + FromPrimitive + AddAssign> {
    fn update(&mut self, x: F);
    fn get(self) -> F;
}

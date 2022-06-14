use num::{Float, FromPrimitive};

pub trait OnlineStatistic<F: Float + FromPrimitive> {
    fn update(&mut self, x: F);
    fn get(self) -> F;
}

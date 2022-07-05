use num::{Float, FromPrimitive};
use std::ops::{AddAssign, SubAssign};
/// Running count.
/// # Examples
/// ```
/// use online_statistics::count::Count;
/// let mut running_count: Count<f64> = Count::new();
/// for i in 1..10{
///     running_count.update(i as f64);
/// }
/// assert_eq!(running_count.get(), 9.0);
/// ```
///
#[derive(Copy, Clone, Default, Debug)]
pub struct Count<F: Float + FromPrimitive + AddAssign + SubAssign> {
    pub count: F,
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> Count<F> {
    pub fn new() -> Self {
        Self {
            count: F::from_f64(0.0).unwrap(),
        }
    }
    #[warn(unused_variables)]
    pub fn update(&mut self, _x: F) {
        self.count += F::from_f64(1.).unwrap();
    }
    pub fn get(&mut self) -> F {
        self.count
    }
}

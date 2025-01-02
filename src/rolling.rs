use crate::stats::{RollableUnivariate, Univariate};
use num::{Float, FromPrimitive};
use std::{
    collections::VecDeque,
    ops::{AddAssign, SubAssign},
};
/// Generic wrapper for performing rolling computations.
/// This can be wrapped around any struct which implements a `Univariate` and a `Revertable` and `RollableUnivariate`
/// traits.
/// Inputs to `update` are stored in a `VecDeque`. Elements of the queue are popped when the window is
/// full.
/// # Arguments
/// * `to_roll` - A running statistics which implements `Univariate` and `Revertable` and `RollableUnivariate` trait.
/// * `window_size` - Size of sliding window.
/// # Examples
/// ```
///
/// use watermill::stats::{RollableUnivariate, Univariate};
/// use watermill::sum::Sum;
/// use watermill::rolling::Rolling;
/// let data = vec![9.,7.,3.,2.,6.,1., 8., 5., 4.];
/// let mut running_sum: Sum<f64> = Sum::new();
/// // We wrap `running_sum` inside the `Rolling` struct.
/// let mut rolling_sum: Rolling<f64> = Rolling::new(& mut running_sum, 2).unwrap();
/// for x in data.iter(){
///     rolling_sum.update(*x as f64);
/// }
/// assert_eq!(rolling_sum.get(), 9.0);
/// ```
pub struct Rolling<'a, F: Float + FromPrimitive + AddAssign + SubAssign> {
    to_roll: &'a mut dyn RollableUnivariate<F>,
    window_size: usize,
    window: VecDeque<F>,
}

impl<'a, F: Float + FromPrimitive + AddAssign + SubAssign> Rolling<'a, F> {
    pub fn new(
        to_roll: &'a mut dyn RollableUnivariate<F>,
        window_size: usize,
    ) -> Result<Self, &'a str> {
        if window_size == 0 {
            return Err("Window size should not equals to 0");
        }
        Ok(Self {
            to_roll,
            window_size,
            window: VecDeque::new(),
        })
    }
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> Univariate<F> for Rolling<'_, F> {
    fn update(&mut self, x: F) {
        if self.window.len() == self.window_size {
            // To handle the error, the program panics because returning the error type would change
            // the interface of the get method. This problem is unlikely to happen because we
            // control the size of the sliding window in the constructor.
            match self.to_roll.revert(*self.window.front().unwrap()) {
                Ok(it) => it,
                Err(err) => panic!("{}", err),
            };
            self.window.pop_front();
            self.window.push_back(x);
        } else {
            self.window.push_back(x);
        }
        self.to_roll.update(x);
    }

    fn get(&self) -> F {
        self.to_roll.get()
    }
}
mod tests {
    #[test]
    fn it_works() {
        use crate::rolling::Rolling;
        use crate::stats::Univariate;
        use crate::variance::Variance;
        let data = vec![9., 7., 3., 2., 6., 1., 8., 5., 4.];
        let mut running_var: Variance<f64> = Variance::default();
        // We wrap `running_var` inside the `Rolling` struct.
        let mut rolling_var: Rolling<f64> = Rolling::new(&mut running_var, 2).unwrap();
        for x in data.iter() {
            rolling_var.update(*x as f64);
        }
        assert_eq!(rolling_var.get(), 0.5);
    }
}

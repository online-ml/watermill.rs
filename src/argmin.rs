use crate::sorted_window::SortedWindow;
use num::{Float, FromPrimitive};
use std::ops::{AddAssign, SubAssign};

use crate::stats::Univariate;
use serde::{Deserialize, Serialize};

/// Rolling argmin.
/// # Arguments
/// * `window_size` - Size of the rolling window.
/// # Examples
/// ```
/// use watermill::argmin::RollingArgMin;
/// use watermill::stats::Univariate;
/// let mut rolling_argmin: RollingArgMin<f64> = RollingArgMin::new(101);
/// for i in 0..=100{
///     rolling_argmin.update(i as f64);
///     //println!("{}", rolling_argmin.get());
///     rolling_argmin.get();
/// }
/// assert_eq!(rolling_argmin.get(), 100.0);
/// ```
///

#[derive(Serialize, Deserialize)]
pub struct RollingArgMin<F: Float + FromPrimitive + AddAssign + SubAssign> {
    sorted_window: SortedWindow<F>,
    window_size: usize,
    argmin: usize,
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> RollingArgMin<F> {
    pub fn new(window_size: usize) -> Self {
        Self {
            sorted_window: SortedWindow::new(window_size),
            window_size,
            argmin: 0,
        }
    }
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> Univariate<F> for RollingArgMin<F> {
    fn update(&mut self, x: F) {
        // Some caching to avoid to search the argmin at each update
        if self.sorted_window.is_empty() {
            self.argmin = 0;
            self.sorted_window.push_back(x);
            return;
        }

        let minimum = self.sorted_window.front();
        self.sorted_window.push_back(x);
        if x > minimum {
            if self.argmin < self.sorted_window.len() - 1 {
                self.argmin += 1;
            } else {
                self.argmin = self
                    .sorted_window
                    .unsorted_window
                    .iter()
                    .rev()
                    .position(|&y| y == x)
                    .expect("Error: argmin not found");
            }
        } else {
            self.argmin = 0;
        }
    }
    fn get(&self) -> F {
        F::from_usize(self.argmin).unwrap()
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    #[test]
    fn rolling_argmin_edge_case() {
        use crate::argmin::RollingArgMin;
        use crate::stats::Univariate;
        let mut rolling_argmin: RollingArgMin<f64> = RollingArgMin::new(3);
        let vec_test = vec![1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 1.5];
        let vec_rolling_argmin = vec![0.0, 1.0, 2.0, 0.0, 0.0, 1.0, 2.0, 0.0];
        for (test_value, truth) in vec_test.iter().zip(vec_rolling_argmin.iter()) {
            rolling_argmin.update(*test_value as f64);
            assert_eq!(rolling_argmin.get(), *truth);
        }
    }
}

use num::Float;

/// Running max.
/// # Examples
/// ```
/// use online_statistics::max::Max;
/// let mut running_max: Max<f64> = Max::new();
/// for i in 1..10{
///     running_max.update(i as f64);
/// }
/// assert_eq!(running_max.get(), 9.0);
/// ```
///
#[derive(Default, Debug)]
pub struct Max<F: Float> {
    pub max: F,
}

impl<F: Float> Max<F> {
    pub fn new() -> Self {
        Self {
            max: F::min_value(),
        }
    }
    pub fn update(&mut self, x: F) {
        if self.max < x {
            self.max = x;
        }
    }
    pub fn get(self) -> F {
        self.max
    }
}

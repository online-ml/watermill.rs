use num::{Float, FromPrimitive};
use ordered_float::NotNan;
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    ops::{AddAssign, Index, SubAssign},
};

#[derive(Serialize, Deserialize)]
pub(crate) struct SortedWindow<F: Float + FromPrimitive + AddAssign + SubAssign> {
    pub(crate) sorted_window: VecDeque<NotNan<F>>,
    pub(crate) unsorted_window: VecDeque<F>,
    window_size: usize,
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> SortedWindow<F> {
    pub fn new(window_size: usize) -> Self {
        Self {
            sorted_window: VecDeque::with_capacity(window_size),
            unsorted_window: VecDeque::with_capacity(window_size),
            window_size,
        }
    }
    pub fn len(&self) -> usize {
        self.sorted_window.len()
    }

    pub fn front(&self) -> F {
        self.sorted_window
            .front()
            .expect("The value is Nan")
            .into_inner()
    }
    pub fn back(&self) -> F {
        self.sorted_window
            .back()
            .expect("The value is NaN")
            .into_inner()
    }
    pub fn push_back(&mut self, value: F) {
        // Before add the newest value to the sorted window
        // we should remove the oldest value
        if self.sorted_window.len() == self.window_size {
            let last_unsorted = self.unsorted_window.pop_front().unwrap();

            let last_unsorted_pos = self
                .sorted_window
                .binary_search(&NotNan::new(last_unsorted).expect("Value is NaN"))
                .expect("The value is Not in the sorted window");
            self.sorted_window.remove(last_unsorted_pos);
        }
        self.unsorted_window.push_back(value);

        let sorted_pos = self
            .sorted_window
            .binary_search(&NotNan::new(value).expect("Value is NaN"))
            .unwrap_or_else(|e| e);
        self.sorted_window
            .insert(sorted_pos, NotNan::new(value).expect("Value is NaN"));
    }
}
impl<F: Float + FromPrimitive + AddAssign + SubAssign> Index<usize> for SortedWindow<F> {
    fn index(&self, index: usize) -> &Self::Output {
        &self.sorted_window[index]
    }
    type Output = F;
}

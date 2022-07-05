use num::{Float, FromPrimitive};
use std::ops::{AddAssign, SubAssign};

use crate::traits::Univariate;

/// Running quantile estimator using P-square Algorithm.
/// # Arguments
/// * `q` - quantile value. **WARNING** Should between `0` and `1`.
/// # Examples
/// ```
/// use online_statistics::quantile::Quantile;
/// use online_statistics::traits::Univariate;
/// let data = vec![9.,7.,3.,2.,5.,6.,1.,8.,4.];
/// let mut running_quantile: Quantile<f64> = Quantile::new(0.5);
/// for x in data.iter(){
///     running_quantile.update(*x as f64);
/// }
/// assert_eq!(running_quantile.get(), 5.0);
/// ```
/// # References
/// [^1]: [The P² Algorithm for Dynamic Univariateal Computing Calculation of Quantiles and Editor Histograms Without Storing Observations](https://www.cse.wustl.edu/~jain/papers/ftp/psqr.pdf)
///
/// [^2]: [P² quantile estimator: estimating the median without storing values](https://aakinshin.net/posts/p2-quantile-estimator/)
#[derive(Clone, Debug)]
pub struct Quantile<F: Float + FromPrimitive + AddAssign + SubAssign> {
    q: F,
    desired_marker_position: Vec<F>,
    marker_position: Vec<F>,
    position: Vec<F>,
    heights: Vec<F>,
    heights_sorted: bool,
}
impl<F: Float + FromPrimitive + AddAssign + SubAssign> Quantile<F> {
    pub fn new(q: F) -> Self {
        if F::from_f64(0.).unwrap() > q && F::from_f64(1.).unwrap() < q {
            panic!("q should be betweek 0 and 1");
        }
        Self {
            q,
            desired_marker_position: vec![
                F::from_f64(0.).unwrap(),
                q / F::from_f64(2.).unwrap(),
                q,
                (F::from_f64(1.).unwrap() + q) / F::from_f64(2.).unwrap(),
                F::from_f64(1.).unwrap(),
            ],
            marker_position: vec![
                F::from_f64(1.).unwrap(),
                F::from_f64(1.).unwrap() + F::from_f64(2.).unwrap() * q,
                F::from_f64(1.).unwrap() + F::from_f64(4.).unwrap() * q,
                F::from_f64(3.).unwrap() + F::from_f64(2.).unwrap() * q,
                F::from_f64(5.).unwrap(),
            ],
            position: (1..6).map(|x| F::from_i32(x).unwrap()).collect(),
            heights: Vec::new(),
            heights_sorted: false,
        }
    }
    fn find_k(&mut self, x: F) -> usize {
        let mut k: Option<usize> = None;
        if x < self.heights[0] {
            self.heights[0] = x;
            k = Some(1);
        } else {
            println!("hello");
            println!("{}", self.heights.len());
            for i in 1..=4 {
                if self.heights[i - 1] <= x && x < self.heights[i] {
                    k = Some(i);
                    break;
                }
            }
            // If k is None it means that the previous loop did not break
            if (*self.heights.last_mut().unwrap() < x) && k.is_none() {
                *self.heights.last_mut().unwrap() = x;
            }
        }
        k.unwrap_or(4)
    }
    fn compute_p2(qp1: F, q: F, qm1: F, d: F, np1: F, n: F, nm1: F) -> F {
        let outer = d / (np1 - nm1);
        let inner_left = (n - nm1 + d) * (qp1 - d) / (np1 - n);
        let inner_right = (np1 - n - d) * (q - qm1) / (n - nm1);
        q + outer * (inner_left + inner_right)
    }

    fn adjust(&mut self) {
        for i in 1..4 {
            let n = self.position[i];
            let q = self.heights[i];
            let mut d = self.marker_position[i] - n;
            if (d >= F::from_f64(1.0).unwrap() && self.position[i] - n > F::from_f64(1.0).unwrap())
                || (d <= F::from_f64(-1.).unwrap()
                    && self.position[i - 1] - n < F::from_f64(-1.).unwrap())
            {
                d = d.copysign(F::from_f64(1.).unwrap());

                let qp1 = self.heights[i + 1];
                let qm1 = self.heights[i - 1];
                let np1 = self.position[i + 1];
                let nm1 = self.position[i - 1];

                let qn = Quantile::compute_p2(qp1, q, qm1, d, np1, n, nm1);

                if qm1 < qn && qn < qp1 {
                    self.heights[i] = qn;
                } else {
                    self.heights[i] = q + d * (self.heights[i - d.to_usize().unwrap()] - q)
                        / (self.position[i + d.to_usize().unwrap()] - n);
                }
                self.position[i] = n + d;
            }
        }
    }
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> Univariate<F> for Quantile<F> {
    fn update(&mut self, x: F) {
        // Initialisation
        if self.heights.len() != 5 {
            self.heights.push(x);
        } else {
            if !self.heights_sorted {
                self.heights.sort_by(|x, y| x.partial_cmp(y).unwrap());
                self.heights_sorted = true;
            }
            // Find cell k such that qk < Xj <= qk+i and adjust extreme values (q1 and q) if necessary
            let k = self.find_k(x);

            // Increment all positions greater than k
            for (index, value) in self.position.iter_mut().enumerate() {
                if index >= k {
                    *value += F::from_f64(1.0).unwrap();
                }
            }

            for (marker, desired_marker) in self
                .marker_position
                .iter_mut()
                .zip(self.desired_marker_position.iter())
            {
                *marker += *desired_marker;
            }
            self.adjust();
        }
    }
    fn get(&mut self) -> F {
        if self.heights_sorted {
            self.heights[2]
        } else {
            self.heights.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let length = F::from_usize(self.heights.len()).unwrap();
            let index = (length - F::from_f64(1.).unwrap())
                .max(F::from_f64(0.).unwrap())
                .min(length * self.q)
                .to_usize()
                .unwrap();

            self.heights[index]
            // TODO: Handle call get during the first value of the stream
        }
    }
}

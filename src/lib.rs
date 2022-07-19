pub mod count;
pub mod covariance;
pub mod ewmean;
pub mod ewvariance;
pub mod iqr;
pub mod kurtosis;
pub mod maximum;
pub mod mean;
pub mod minimum;
pub mod moments;
pub mod ptp;
pub mod quantile;
pub mod rolling;
pub mod skew;
mod sorted_window;
pub mod sum;
pub mod traits;
pub mod variance;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

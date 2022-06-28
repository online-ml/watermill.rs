pub mod count;
pub mod kurtosis;
pub mod maximum;
pub mod mean;
pub mod minimum;
pub mod moments;
pub mod skew;
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

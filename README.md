# Online statistics in Rust 🦀 

**`online-statistics` is crate 🦀 for Blazingly fast ⚡️, generic 🏭 and serializable 📝, online 🌊 statistics 📊.**

## Quickstart
---------
Let's compute the online median and then serialize it:
```rust
use online_statistics::quantile::Quantile;
use online_statistics::traits::Univariate;
let data = vec![9., 7., 3., 2., 6., 1., 8., 5., 4.];
let mut running_median: Quantile<f64> = Quantile::new(0.5_f64).unwrap();
for x in data.iter() {
    running_median.update(*x as f64); // update the current statistics
    println!("The actual median value is: {}", running_median.get());
}
assert_eq!(running_median.get(), 5.0);

// Convert the statistic to a JSON string.
let serialized = serde_json::to_string(&running_median).unwrap();

// Convert the JSON string back to a statistic.
let deserialized: Quantile<f64> = serde_json::from_str(&serialized).unwrap();

```
Now let's compute the online sum using the iterators:
```rust
use online_statistics::iter::IterStatExt;
let data: Vec<f64> = vec![1., 2., 3.];
let vec_true: Vec<f64> = vec![1., 3., 6.];
for (d, t) in data.into_iter().online_sum().zip(vec_true.into_iter()) {
    assert_eq!(d, t);
}
```

You can also compute rolling statistics, in the following example let's compute the rolling sum on 2 previous data: 
```rust

use online_statistics::traits::{RollableUnivariate, Univariate};
use online_statistics::sum::Sum;
use online_statistics::rolling::Rolling;
let data = vec![9.,7.,3.,2.,6.,1., 8., 5., 4.];
let mut running_sum: Sum<f64> = Sum::new();
// We wrap `running_sum` inside the `Rolling` struct.
let mut rolling_sum: Rolling<f64> = Rolling::new(& mut running_sum, 2).unwrap();
for x in data.iter(){
    rolling_sum.update(*x as f64);
}
assert_eq!(rolling_sum.get(), 9.0);
```

## Installation
---------
Add the following line to your `cargo.toml`:
```
[dependencies]
online-statistics = "0.1.0"
```

## Statistics available
| Statistics                      	| Rollable ?|
|---------------------------------	|----------	|
| Mean                            	| ✅        	|
| Variance                        	| ✅        	|
| Sum                             	| ✅        	|
| Min                             	| ✅        	|
| Max                             	| ✅        	|
| Count                           	| ❌        	|
| Quantile                        	| ✅        	|
| Peak to peak                    	| ✅        	|
| Exponentially weighted mean     	| ❌        	|
| Exponentially weighted variance 	| ❌        	|
| Interquartile range             	| ✅        	|
| Kurtosis                        	| ❌        	|
| Skewness                        	| ❌        	|
| Covariance                      	| ❌        	|

## Inspiration
---------
The `stats` module of the [`river`](https://github.com/online-ml/river) library in `Python` greatly inspired this crate. 
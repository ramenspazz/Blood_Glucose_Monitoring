use rayon::prelude::*;

pub fn mean(data: &[f64]) -> Result<f64, String> {
    match data.len() {
        count if count > 0 => {
            let sum: f64 = data.par_iter().map(|item| item).sum::<f64>();
            let mean = sum as f64 / count as f64;
            Ok(mean)
        }
        _ => Err("There is not enough points to compute the mean!".to_string()),
    }
}

/// Purpose
/// -------
/// Multithreaded implementation of the sample standard deviation of an array/vector of float 64 numbers.
/// Uses the multithreaded implementation of `mean` as well.
/// The data type of mean must be castable to f64.
pub fn std_deviation(data: &[f64], mean: f64) -> Result<f64, String> {
    match data.len() {
        num_datapoints if num_datapoints > 1 => {
            let variance = data
                .par_iter()
                .map(|value| {
                    let diff = mean - value;
                    diff * diff
                })
                .sum::<f64>()
                / ((num_datapoints - 1) as f64);

            Ok(variance.sqrt())
        }
        _ => Err("There are not enough datapoints to calculate the standard deviation!".to_string()),
    }
}

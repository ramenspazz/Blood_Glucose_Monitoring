use rayon::prelude::*;

pub fn mean(data: &[(f64, f64)]) -> Option<f64> {
    match data.len() {
        count if count > 0 => {
            let sum: f64 = data.par_iter().map(|item| item.1).sum();
            let mean = sum as f64 / count as f64;
            Some(mean)
        }
        _ => None,
    }
}

/// Purpose
/// -------
/// Multithreaded implementation of the sample standard deviation of an array/vector of float 64 numbers.
/// Uses the multithreaded implementation of `mean` as well.
/// The data type of mean must be castable to f64.
pub fn std_deviation(data: &[(f64, f64)], mean: f64) -> Option<f64> {
    match data.len() {
        count if count > 0 => {
            let variance = data
                .par_iter()
                .map(|value| {
                    let diff = mean - value.1;
                    diff * diff
                })
                .sum::<f64>()
                / ((count - 1) as f64);

            Some(variance.sqrt())
        }
        _ => None,
    }
}

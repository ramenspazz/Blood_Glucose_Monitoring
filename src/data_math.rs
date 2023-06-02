use rayon::prelude::*;

pub fn mean(data: &Vec<u16>) -> Option<f64> {
    let sum = data.par_iter().sum::<u16>() as usize;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum as f64 / count as f64),
        _ => None,
    }
}

/// Purpose
/// -------
/// Multithreaded implementation of the sample standard deviation of a array/vector of float 64 numbers.
/// Uses the multihreaded implementation of `mean` as well.
/// The data type of mean must be castable to f64.
pub fn std_deviation(data: &Vec<u16>, mean: u32) -> Option<f64> {
    match data.len() {
        count if count > 0 => {
            let variance = data
                .par_iter()
                .map(|value| {
                    let diff = (mean as f64) - (*value as f64);

                    diff * diff
                })
                .sum::<f64>()
                / ((count - 1) as f64);

            Some(variance.sqrt())
        }
        _ => None,
    }
}
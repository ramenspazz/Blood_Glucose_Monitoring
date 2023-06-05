use crate::data_point::DataTrack;
use std::fmt::Debug;

// Function to compute the Levenberg-Marquardt fit for the passed expression
pub fn compute_lm_fit<F, R>(data: &[DataTrack], expression: F) -> Vec<R>
where
    F: Fn(DataTrack) -> R,
    R: Debug,
{
    // Create a vector to store the accumulated results
    let mut results = Vec::new();

    // Iterate over the data tracks and compute the results
    for data_track in data {
        let result = expression(data_track.clone());
        results.push(result);
    }

    // Print the results
    println!("Results: {:.2?}", results);
    results
}

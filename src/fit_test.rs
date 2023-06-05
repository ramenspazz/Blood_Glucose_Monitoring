#[cfg(test)]
mod tests {
    use crate::data_point::DataTrack;
    use std::fmt::Debug;

    // Function to compute the Levenberg-Marquardt fit for the passed expression
    pub fn compute_lm_fit<F, R>(data: &[DataTrack], expression: F)
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
        println!("Results: {:?}", results);
    }

    #[test]
    fn test_compute_lm_fit() {
        // Define the driver expression
        let expression = |data_track: DataTrack| -> f64 {
            // Extract the relevant input variables
            let long_units_rendered = data_track
                .long_units_rendered_vec
                .iter()
                .map(|(x, _)| x)
                .sum::<f64>();
            let fast_units_rendered = data_track
                .fast_units_rendered_vec
                .iter()
                .map(|(x, _)| x)
                .sum::<f64>();
            let food_eaten = data_track
                .food_eaten_vec
                .iter()
                .map(|(_, y)| y)
                .sum::<f64>();

            // Compute the blood glucose level based on the input variables
            let blood_glucose_level =
                100.0 + (long_units_rendered * 50.0) + (fast_units_rendered * 30.0) + (food_eaten * 200.0);

            blood_glucose_level
        };

        // Create sample data
        let data: Vec<DataTrack> = vec![
            DataTrack {
                blood_glucose_vec: vec![],
                long_units_rendered_vec: vec![(1.0, 2.0), (3.0, 4.0)],
                fast_units_rendered_vec: vec![(0.5, 1.0), (1.0, 2.0)],
                food_eaten_vec: vec![(0.2, 0.5), (0.3, 0.6)],
                day: 0,
            },
            DataTrack {
                blood_glucose_vec: vec![],
                long_units_rendered_vec: vec![(5.0, 6.0), (7.0, 8.0)],
                fast_units_rendered_vec: vec![(2.0, 2.5), (1.5, 2.0)],
                food_eaten_vec: vec![(1.0, 1.5), (2.0, 1.5)],
                day: 0,
            },
        ];

        // Call the compute_lm_fit function
        compute_lm_fit(&data, expression);
    }
}

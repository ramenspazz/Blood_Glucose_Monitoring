#[macro_use]
pub mod proj_macros;
use std::fs::File;
pub mod data_fit;
pub mod data_math;
pub mod data_parsing;
pub mod data_point;
pub mod plot_data;
pub mod readin;

fn main() {
    let mut file: File = {
        match data_parsing::openfile() {
            Some(valid_file) => valid_file,
            None => return,
        }
    };

    // manipulate data into a series of datatracks to be added
    let data = data_parsing::parse_data(&mut file);
    plot_data::plot_data(&data);
    // let mut data_fit = expression(&data);

    // println!["{:.4?}", &data_fit];

    // let weighted_avg: f64 = {
    //     std::iter::zip(&data, &transformed_data)
    //         .map(|(a, b)| (a.blood_glucose_vec.len() as f64) * b)
    //         .sum::<f64>()
    //         /
    //         data
    //             .iter()
    //             .map(|a| a.blood_glucose_vec.len() as f64)
    //             .sum::<f64>()
    // };
    // println!("Weighted average: {:.2}", weighted_avg);
}

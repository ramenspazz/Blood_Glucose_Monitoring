#[macro_use]
pub mod proj_macros;
use std::fs::File;
pub mod readin;
pub mod data_math;
pub mod data_point;
pub mod data_parsing;
pub mod plot_data;

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
}

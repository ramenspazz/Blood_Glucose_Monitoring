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
    
    // TODO: save a representation of the data to a file that can be streamed into 
    // memory and construct the object to more quickly load a previously computed entry, else compute newly?
    
    // manipulate data into a series of datatracks to be added to the master track
    let data = data_parsing::parse_data(&mut file);
    plot_data::plot_data(&data);
    let mut master_track_vec: Vec<Vec<Vec<f64>>> = Vec::with_capacity(0xFFFF);

    for track in data.iter() {
        let mut diff_vec: Vec<Vec<f64>> = vec![];
        if track.data_tensor.len() > 1 {
            for i in 0..(track.data_tensor.len() - 1) {
                diff_vec.push(
                    track
                        .d_tensor(i, i+1)
                        .unwrap()
                );
            }
            master_track_vec.push(diff_vec.clone());
        }
    }

    println!["delta(mg/dL) / delta(hours), delta(mg/dL)/(units u40 taken last time), delta(mg/dL)/(units u100 taken last time)"];
    for (i,data_track) in master_track_vec.iter().enumerate() {
        if data_track.len() > 1 {
            println!["Track {}:", i + 1];
            for records in data_track.iter() {
                println!["{:.4?}", records];
            }
        }
    }
}

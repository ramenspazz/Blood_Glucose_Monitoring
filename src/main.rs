#[macro_use]
pub mod getln_macro;
use std::fs::File;
pub mod readin;
pub mod data_math;
pub mod data_point;
pub mod data_parsing;


use plotly::common::{Title};
use plotly::layout::{Axis, Layout};
use plotly::{Plot, Scatter};

fn main() {
    let mut file: File = {
        match data_parsing::openfile() {
            Some(valid_file) => valid_file,
            None => return,
        }
    };

    let temp_ = data_parsing::parse_data(&mut file);
    let data = temp_.0;
    let size = temp_.1;

    println!["total number of entries: {}", &size];
    for (_i, track) in data.iter().enumerate() {
        let daily_mean_blood_glucose = data_math::mean(&track.blood_glucose_vec).unwrap();
        match data_math::std_deviation(&track.blood_glucose_vec, daily_mean_blood_glucose as u32) {
            Some(std_dev) => 
            {
                println!["Hana's average bgl for day {} with {} measurement(s) is {}, with a S.D. of {}.", &track.day, &track.blood_glucose_vec.len(), &daily_mean_blood_glucose, &std_dev];
                println!["Units taken: {:?}", &track.units_rendered_vec];
            },
            None => { println!["ERROR : could not process data! Exiting...\n"]; return },
        }
    }
    
    let layout = Layout::new()
                            .x_axis(Axis::new().title(Title::from("time(minutes out of 1400)")))
                            .y_axis(Axis::new().title(Title::from("mg/dL")))
                            .title(Title::from("Hana's blood glucose levels"));

    
    let mut plot = Plot::new();
    for (_i, track) in data.iter().enumerate() {
        let trace_n: Box<Scatter<u16, u16>> = {
            Scatter::new(track.time_taken_vec.clone(), track.blood_glucose_vec.clone())
                .name(format!["Day {}", &track.day])
        };
        plot.add_trace(trace_n);
    }
    plot.set_layout(layout);
    plot.show();

    // this is a possible model I can use to fit data to.
    // in this equation, f(t) represents if at time t food was consumed or not, taking a binary value of 0 or 1.
    // u(t) represents how many units were administered at time t.
    //
    // d²u/dt² = -k₁ * du/dt + k₂ * (f(t) - u)
    //
    // TODO : numerically fit this equation.

}

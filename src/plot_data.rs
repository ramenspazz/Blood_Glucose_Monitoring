use crate::data_math;
use crate::data_point::DataTrack;

use plotlib::repr::Plot;
use plotlib::style::{PointStyle, LineStyle, LineJoin, PointMarker};
use plotlib::view::ContinuousView;
use plotlib::page::Page;

use rand::Rng;


fn generate_random_color() -> String {
    let mut rng = rand::thread_rng();
    let color: String = (0..6)
        .map(|_| rng.gen_range(0..=15))
        .map(|num| format!("{:X}", num))
        .collect();
    format!("#{}", color)
}

pub fn plot_data(data: &Vec<DataTrack>) {
    let mut v = ContinuousView::new()
        .x_range(0., 24.)
        .y_range(0., 650.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    let total_entries = {
        let _temp = 0;
        for track in data {
            let mut _temp = _temp + track.blood_glucose_vec.len();
        }
        _temp
    };
    println!["total number of entries: {}", &total_entries];
    for (_i, track) in data.iter().enumerate() {
        let cur_colour = generate_random_color();
        let plot = Plot::new(track.blood_glucose_vec.clone())
            .point_style(
                PointStyle::new()
                    .marker(PointMarker::Square)
                    .colour(&cur_colour),
            )
            .line_style(
                LineStyle::new()
                    .colour(&cur_colour)
                    .linejoin(LineJoin::Round),
            );

        let daily_mean_blood_glucose = data_math::mean(&track.blood_glucose_vec).unwrap();
        match data_math::std_deviation(&track.blood_glucose_vec, daily_mean_blood_glucose) {
            Some(std_dev) => {
                println!(
                    "Hana's average bgl for day {} with {} measurement(s) is {}, with a S.D. of {}.",
                    &track.day,
                    &track.blood_glucose_vec.len(),
                    &daily_mean_blood_glucose,
                    &std_dev
                );
                println!(
                    "Units taken (time in seconds from 00:00, units):\nlong: {:?}\nfast: {:?}",
                    &track.long_units_rendered_vec,
                    &track.fast_units_rendered_vec,
                );
            }
            None => {
                println!["ERROR: could not process data! Exiting...\n"];
                return;
            }
        }

        v = v.add(plot);
    }

    // now add units taken data ontop of BGL data

    // let mut units_track = 

    // v = v.add(todo![]);

    Page::single(&v).save("scatter.svg").unwrap();
}
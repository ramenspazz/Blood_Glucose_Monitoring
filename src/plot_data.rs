use crate::data_point::DataTrack;
use crate::data_math;
use plotters::prelude::*;

use rand::Rng;

fn generate_random_color() -> RGBColor {
    let mut rng = rand::thread_rng();
    RGBColor(
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
    )
}

pub fn plot_data(data: &Vec<DataTrack>) {
    let drawing_area = BitMapBackend::new("scatter.png", (1920, 1080)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..24, 0..650)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(24)
        .y_labels(32)
        .x_desc("Time (hours)")
        .y_desc("Blood Glucose level (mg/dL=>1E-02g/L)")
        .draw()
        .unwrap();

    let total_entries = data.iter().map(|track| track.blood_glucose_vec.len()).sum::<usize>();
    println!("Total number of entries: {}", total_entries);

    let mut legend_labels = Vec::new();

    for track in data {
        let cur_colour: RGBColor = generate_random_color();

        let points: Vec<(i32, i32)> = track
            .blood_glucose_vec
            .iter()
            .enumerate()
            .map(|(i, &(x, y))| (x as i32, y as i32))
            .collect();

        // Create the LineSeries with the modified points
        let series = LineSeries::new(points, &cur_colour);

        // Draw the LineSeries
        chart
            .draw_series(series)
            .unwrap();

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
                    "Units taken (time in hours from 00:00, units):\nlong: {:?}\nfast: {:?}",
                    &track.long_units_rendered_vec,
                    &track.fast_units_rendered_vec,
                );

                // Add legend label
                legend_labels.push((track.day.to_string(), cur_colour));
            }
            None => {
                println!("ERROR: could not process data! Exiting...\n");
                return;
            }
        }
    }

    use plotters::prelude::*;

    // Draw legend
    let drawing_area = chart.plotting_area().clone();
    for (i, &(ref label, color)) in legend_labels.iter().enumerate() {
        let rect_width = 1;                             // Width of the legend color rectangle
        let rect_height = 20;                           // Height of the legend color rectangle
        let rect_x = 2;                                 // X coordinate of the top-left corner of the rectangle
        let rect_y = (i * 25 + 5 + 256 + 128) as i32;   // Y coordinate of the top-left corner of the rectangle
        let text_x = 2;                             // X coordinate of the legend label
        let text_y = (i * 25 + 21 + 256 + 128) as i32;  // Y coordinate of the legend label

        let formatted_label = format!("Day {}", label);  // Add "Day" prefix to the label

        drawing_area
            .draw(
                &Rectangle::new(
                    [(rect_x, rect_y), (rect_x + rect_width, rect_y + rect_height)],
                    color.filled(),
                )
            )
            .unwrap();

        drawing_area
            .draw(
                &Text::new(
                    formatted_label,  // Use the formatted label with "Day" prefix
                    (text_x, text_y),
                    ("sans-serif", 15).into_font(),
                )
            )
            .unwrap();
    }

    // Position the legend in the top right corner
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .draw()
        .unwrap();
}

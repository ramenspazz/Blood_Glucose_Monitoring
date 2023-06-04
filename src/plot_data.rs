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

    let mut asdf = ChartBuilder::on(&drawing_area);
    let chart_builder = asdf
        .margin(10)
        .set_left_and_bottom_label_area_size(20);
        
    let mut chart_context = chart_builder.build_cartesian_2d(0.0..24.0, 0.0..650.0).unwrap();
    chart_context.configure_mesh()
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

        let points: Vec<(f64,f64)> = track
            .blood_glucose_vec
            .iter()
            .enumerate()
            .map(|(_i, &(x, y))| (x as f64, y as f64))
            .collect();

        // Create the LineSeries with the modified points
        let series = LineSeries::new(points, &cur_colour);
        use plotters::prelude::*;
        // Draw the LineSeries
        chart_context
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

    // Draw legend
    let drawing_area = chart_context.plotting_area().clone();
    for (i, &(ref label, color)) in legend_labels.iter().enumerate() {
        let rect_width: f64 = 1f64;                             // Width of the legend color rectangle
        let rect_height: f64 = 20f64;                           // Height of the legend color rectangle
        let rect_x: f64 = 2f64;                                 // X coordinate of the top-left corner of the rectangle
        let rect_y = (i * 25 + 5 + 256 + 128) as f64;   // Y coordinate of the top-left corner of the rectangle
        let text_x: f64 = 2f64;                                 // X coordinate of the legend label
        let text_y = (i * 25 + 21 + 256 + 128) as f64;  // Y coordinate of the legend label

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
    chart_context
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .draw()
        .unwrap();
}

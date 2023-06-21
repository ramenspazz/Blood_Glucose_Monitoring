use crate::data_point::DataTrack;
use crate::data_math;
use plotters::prelude::*;
use rand::Rng;

fn generate_random_color() -> RGBColor {
    let mut rng = rand::thread_rng();
    RGBColor(
        rng.gen_range(10u8..=245u8),
        rng.gen_range(10u8..=245u8),
        rng.gen_range(10u8..=245u8),
    )
}

fn is_similar_color(color1: RGBColor, color2: RGBColor, tolerance: i16) -> bool {
    let red_diff = (color1.0 as i16) - (color2.0 as i16);
    let green_diff = (color1.1 as i16) - (color2.1 as i16);
    let blue_diff = (color1.2 as i16) - (color2.2 as i16);

    (red_diff.abs() <= tolerance) && (green_diff.abs() <= tolerance) && (blue_diff.abs() <= tolerance)
}

pub fn plot_data(data: &Vec<DataTrack>) {
    let drawing_area = BitMapBackend::new("scatter.png", (3840/2, 2160/2)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    
    let mut asdf = ChartBuilder::on(&drawing_area);
    let chart_builder = asdf
        .margin(64)
        .set_left_and_bottom_label_area_size(48);
        
    let mut chart_context = chart_builder.build_cartesian_2d(0.0..24.0, 0.0..650.0).unwrap();
    chart_context
        .configure_mesh()
        .x_labels(24)
        .y_labels(32)
        .x_desc("Time (hours)")
        .y_desc("Blood Glucose level (mg/dL=>1E-02g/L)")
        .draw()
        .unwrap();

    let total_entries = data.iter().map(|track| track.data_tensor.len()).sum::<usize>();
    println!("Total number of entries: {}", total_entries);
    
    let mut legend_labels = Vec::new();
    let mut cur_color;
    let mut color_vec = Vec::with_capacity(0xFFFF);
    for track in data {
        cur_color = {
            'colour_selector: loop {
                cur_color = generate_random_color();
                if color_vec.is_empty() {
                    color_vec.push(cur_color.clone());
                    break 'colour_selector;
                } else {
                    let is_similar = color_vec.iter().any(|&existing_color| is_similar_color(cur_color, existing_color, 15));
                    if is_similar {
                        continue 'colour_selector;
                    } else {
                        color_vec.push(cur_color.clone());
                        break 'colour_selector;
                    }
                }
            }
            cur_color
        };
        
        let points_series: Vec<(f64,f64)> = track
            .data_tensor
            .iter()
            .map(|(t, bgl, _, _, _)| (*t as f64, *bgl as f64))
            .collect::<Vec<(f64,f64)>>();

        let points: Vec<f64> = points_series
            .iter()
            .map(|(_,bgl)| *bgl as f64)
            .collect::<Vec<f64>>();

        let u40: Vec<(f64,f64)> = track
            .data_tensor
            .iter()
            .map(|(t,_,u40_units,_,_)| (*t as f64, *u40_units as f64))
            .collect::<Vec<(f64,f64)>>();

        let u100: Vec<(f64,f64)> = track
            .data_tensor
            .iter()
            .map(|(t,_,_,u100_units,_)| (*t as f64, *u100_units as f64))
            .collect::<Vec<(f64,f64)>>();
        
        let series = LineSeries::new(points_series.clone(), &cur_color).point_size(5);
        
        chart_context.draw_series(series).unwrap();

        let daily_mean_blood_glucose = data_math::mean(&points).unwrap();
        match data_math::std_deviation(&points, daily_mean_blood_glucose) {
            Ok(std_dev) => {
                println!(
                    "Hana's average bgl for day {} with {} measurement(s) is {:.2}, with a S.D. of {:.2}.",
                    &track.day,
                    &track.data_tensor.len(),
                    &daily_mean_blood_glucose,
                    &std_dev
                );
                println!(
                    "Units taken (time in hours from the begining of the same day, units):\nlong: {:.2?}\nfast: {:.2?}",
                    &u40,
                    &u100,
                );

                // Add legend label
                legend_labels.push((track.day.to_string(), cur_color));
            }
            Err(e) => {
                println!("ERROR: could not process data!\n{}\n", e);
                return;
            }
        }
    }

    // Draw legend
    let drawing_area = chart_context.plotting_area().clone();
    for (i, &(ref label, color)) in legend_labels.iter().enumerate() {
        let rect_width: f64 = 0.5f64;                            // Width of the legend color rectangle
        let rect_height: f64 = 12f64;                            // Height of the legend color rectangle
        let rect_x: f64 = 1.5f64;                                // X coordinate of the top-left corner of the rectangle
        let rect_y = (i * 25 + 5 + 256 + 128) as f64;       // Y coordinate of the top-left corner of the rectangle
        let text_x: f64 = 2.125f64;                              // X coordinate of the legend label
        let text_y = (i * 25 + 18 + 256 + 128) as f64;      // Y coordinate of the legend label

        let formatted_label: String = format!("Day {}", label);  // Add "Day" prefix to the label

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
                    ("sans-serif", 16).into_font(),
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

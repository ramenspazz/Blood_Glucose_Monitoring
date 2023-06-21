use crate::data_point;
use crate::readin;
use std::io::Write;
use std::path::Path;

const MAX_SIZE: usize = 0xFFFF_FFFF_FFFF_FFFF;
const BITS_16: usize = 0xFFFF;

pub fn openfile() -> Option<std::fs::File> {
    loop {
        // Uncomment out these lines and delete the line following to enable being able to select your own file
        // Create a path to the desired file
        // println!["Enter the path to data, or ..exit to exit: "];
        // let path_str = readln![];
        // if path_str.trim() == "..exit" {
        //     return None
        // }
        let path_str = "insulin.dat";
        let path: &Path = Path::new({
            let binding: &str = path_str.trim();
            binding
        }); // create and bind a value and assign it to path
        let _display: std::path::Display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut _file: std::fs::File = match std::fs::File::open(&path) {
            Err(_) => continue,
            Ok(file) => return Option::Some(file),
        };
    }
}

pub fn parse_data(file: &mut std::fs::File) -> Vec<data_point::DataTrack> {
    let mut data_track_by_day_vec: Vec<data_point::DataTrack> = Vec::with_capacity(BITS_16);
    let mut _cur_day: u16 = 0;
    let mut last_day: u16 = 0;
    let mut first_run: bool = true;

    loop {
        let mut cur_data_track: data_point::DataTrack = data_point::DataTrack::new();
        let mut blood_glucose_1: f64 = 0f64;
        let mut u100_units_1: f64 = 0f64;
        let mut u40_units_1: f64 = 0f64;
        let mut food_eaten: f64 = 0f64;
        let mut time_1: f64 = 0f64;

        let mut was_working: bool = false;

        'a: for _ in 0..MAX_SIZE {
            // this loop will go for MAX_SIZE data rows total
            for j in 0..6 {
                // this loop collects a current rows data
                match readin::readin(file) {
                    Some(number) => {
                        match j {
                            0 => {
                                blood_glucose_1 = number as f64;
                                was_working = true;
                            }
                            1 => u40_units_1 = number as f64,
                            2 => u100_units_1 = number as f64,
                            3 => food_eaten = number as f64,
                            4 => time_1 = (number / 3600.0) as f64,
                            5 => {
                                _cur_day = number as u16;
                                if first_run {
                                    last_day = _cur_day;
                                    first_run = false;
                                } else if _cur_day != last_day {
                                    data_track_by_day_vec.push(cur_data_track);
                                    // create a new data track for the new day
                                    cur_data_track = data_point::DataTrack::new();
                                    last_day = _cur_day;
                                }

                                // push this rows data to the data track
                                cur_data_track.data_tensor.push((
                                    time_1,
                                    blood_glucose_1,
                                    u40_units_1,
                                    u100_units_1,
                                    food_eaten,
                                ));
                                cur_data_track.day = _cur_day;
                            }
                            _ => {}
                        }
                    }
                    // we are now done reading data and break the main loop
                    None => {
                        // if this is the last entry, push it to the data vec and then return
                        if was_working == true {
                            data_track_by_day_vec.push(cur_data_track);
                        }
                        break 'a;
                    }
                }
            }
        }
        return data_track_by_day_vec;
    }
}

pub fn write_to_file<T>(format_str: String, numbers: Vec<T>) -> std::io::Result<()>
where
    T: ToString,
{
    let mut file = std::fs::File::create("output.txt")?;

    let mut output_str = String::new();
    let mut number_index = 0;

    for ch in format_str.chars() {
        if ch == '#' {
            if let Some(number) = numbers.get(number_index) {
                output_str.push_str(&number.to_string());
                number_index += 1;
            }
        } else {
            output_str.push(ch);
        }
    }

    writeln!(file, "{}", output_str)?;

    Ok(())
}

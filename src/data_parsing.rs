use crate::data_point;
use crate::readin;
use std::path::Path;

const MAX_SIZE: usize = 0xFFFF_FFFF_FFFF_FFFF;
const BITS_16: usize = 0xFFFF;

pub fn openfile() -> Option<std::fs::File> {
    loop {
        // Create a path to the desired file
        // println!["Enter the path to data, or ..exit to exit: "];
        // let path_str = getln![];
        // if path_str.trim() == "..exit" {
        //     return None
        // }
        let path_str = "insulin.csv";
        let path: &Path = Path::new({let binding: &str = path_str.trim(); binding}); // create and bind a value and assign it to path
        let _display: std::path::Display = path.display();
    
        // Open the path in read-only mode, returns `io::Result<File>`
        let mut _file: std::fs::File = match std::fs::File::open(&path) {
            Err(_) => continue,
            Ok(file) => return Option::Some(file),
        };
    }
}

pub fn parse_data(file: &mut std::fs::File) -> (Vec<data_point::DataTrack>, usize) {
    let mut data_track_by_day_vec: Vec<data_point::DataTrack> = Vec::with_capacity(BITS_16);
    let mut _cur_day: u16 = 0;
    let mut last_day: u16 = 0;
    let mut first_run: bool = true;
    let mut total_size: usize = 0;

    loop {
        let mut cur_data_track: data_point::DataTrack = data_point::DataTrack::new();
        let mut temp_blg1: u16 = 0;
        let mut temp_units1: f64 = 0.0;
        let mut temp_time1: u16 = 0;
        let mut _temp_blg2: u16 = 0;
        let mut _temp_units2: f64 = 0.0;
        let mut _temp_time2: u16 = 0;
        let mut was_working: bool = false;

        'a: for _ in 0..MAX_SIZE { // this loop will go for MAX_SIZE data rows total
            for j in 0..5 { // this loop collects a current rows data
                match readin::readin(file) {
                    Some(number) => {
                        match j {
                            0 => temp_blg1 = number as u16,
                            1 => temp_units1 = number as f64,
                            3 => temp_time1 = (number / 60.0) as u16,
                            4 => {
                                was_working = true;
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
                                total_size += 1;
                                _temp_blg2 = temp_blg1;
                                _temp_units2 = temp_units1;
                                _temp_time2 = temp_time1;
                                // push this rows data to the data track
                                cur_data_track.blood_glucose_vec.push(temp_blg1);
                                cur_data_track.units_rendered_vec.push(temp_units1);
                                cur_data_track.time_taken_vec.push(temp_time1);
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
                        break 'a
                    },
                }
            }
        }
        return (data_track_by_day_vec, total_size)
    }
}

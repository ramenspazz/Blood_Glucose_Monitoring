const BITS_16: usize = 0xFFFF;

#[derive(Debug)]
pub struct DataTrack {
    pub blood_glucose_vec: Vec<u16>,
    pub units_rendered_vec: Vec<f64>,
    pub time_taken_vec: Vec<u16>,
    pub day: u16,
}

impl DataTrack {
    pub fn new() -> Self {
        Self {
            blood_glucose_vec: Vec::with_capacity(BITS_16),
            units_rendered_vec: Vec::with_capacity(BITS_16),
            time_taken_vec: Vec::with_capacity(BITS_16),
            day: 0,
        }
    }
}

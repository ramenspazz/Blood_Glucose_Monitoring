const BITS_16: usize = 0xFFFF;

#[derive(Debug)]
pub struct DataTrack {
    pub blood_glucose_vec: Vec<(f64,f64)>,
    pub long_units_rendered_vec: Vec<(f64,f64)>,
    pub fast_units_rendered_vec: Vec<(f64,f64)>,
    pub food_eaten_vec: Vec<u8>,
    pub day: u16,
}

impl DataTrack {
    pub fn new() -> Self {
        Self {
            blood_glucose_vec: Vec::with_capacity(BITS_16),
            long_units_rendered_vec: Vec::with_capacity(BITS_16),
            fast_units_rendered_vec: Vec::with_capacity(BITS_16),
            food_eaten_vec: Vec::with_capacity(BITS_16),
            day: 0,
        }
    }
}

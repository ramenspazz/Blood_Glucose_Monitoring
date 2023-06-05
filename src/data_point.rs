const BITS_16: usize = 0xFFFF;

#[derive(Debug, Clone)]
pub struct DataTrack {
    // data is stored internally as
    // time | BGL | U40 | U100 | food eaten
    pub data_tensor: Vec<(f64, f64, f64, f64, f64)>,
    pub day: u16,
}

impl DataTrack {
    pub fn new() -> Self {
        Self {
            data_tensor: Vec::with_capacity(BITS_16),
            day: 0,
        }
    }
}

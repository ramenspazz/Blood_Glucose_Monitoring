use std::thread;
const BITS_16: usize = 0xFFFF;

/// # Purpose
/// A structure to contain a R^(nx5) tensor of data.
/// data is stored internally as:
/// 
/// time | BGL | U40 | U100 | food eaten
#[derive(Debug, Clone)]
pub struct DataTrack {
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

    /// # Purpose
    /// Compute the finite difference vector between two data points `range_start` and `range_start`.
    /// The function requires that `range_start` is strictly less than `range_end`, as the reverse calling
    /// would be off by a negative only. Ex: d_tensor(a,b) = -d_tensor(b, a).
    pub fn d_tensor(&self, range_start: usize, range_end: usize) -> Result<Vec<f64>, String> {
        if self.data_tensor.len() <= 1 {
            return Err("Insufficient data points".to_string());
        } else if range_start == range_end || range_end >= self.data_tensor.len() {
            return Err(format!("Invalid range! Max index is {}; Got, [{}, {}]", self.data_tensor.len() - 1, range_start, range_end).to_string());
        }

        let vec_end_ref = match self.data_tensor.get(range_end) {
            Some(value) => *value,
            None => return Err(format!("Invalid range! Max index is {}; Got, [{}, {}]", self.data_tensor.len() - 1, range_start, range_end).to_string()),
        };

        let vec_start_ref = match self.data_tensor.get(range_start) {
            Some(value) => *value,
            None => return Err(format!("Invalid range! Max index is {}; Got, [{}, {}]", self.data_tensor.len() - 1, range_start, range_end).to_string()),
        };

        let delta_time_end_start = self.data_tensor[range_end].0 - self.data_tensor[range_start].0;

        let handle1 = thread::spawn(move || {
            (vec_end_ref.1 - vec_start_ref.1) / delta_time_end_start
        });

        let handle2 = thread::spawn(move || {
            (vec_end_ref.2 - vec_start_ref.2) / delta_time_end_start
        });

        let handle3 = thread::spawn(move || {
            (vec_end_ref.3 - vec_start_ref.3) / delta_time_end_start
        });

        let result1 = handle1.join().map_err(|e| format!("Thread 1 panicked: {:?}", e))?;
        let result2 = handle2.join().map_err(|e| format!("Thread 2 panicked: {:?}", e))?;
        let result3 = handle3.join().map_err(|e| format!("Thread 3 panicked: {:?}", e))?;

        Ok(vec![result1, result2, result3])
    }
}

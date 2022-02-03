//! Time duration consisting in a number of a given bar divisions

/// Time duration consisting in a number of a given bar divisions
#[derive(Clone, Copy, Debug)]
pub struct Duration {
    pub divisions: u32,
    pub length: u32,
}

impl Duration {
    pub fn new(divisions: u32, length: u32) -> Self {
        Duration { divisions, length }
    }

    // Convert duration into seconds
    pub fn to_seconds(&self, bpm: f64, bpb: u32) -> f64 {
        let bar_duration = (bpb as f64) * 60. / bpm;
        self.length as f64 * bar_duration / self.divisions as f64
    }
}

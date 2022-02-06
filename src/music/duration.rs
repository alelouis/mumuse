//! Time duration consisting in a number of a given bar divisions

/// Time duration consisting in a number of a given bar divisions
#[derive(Clone, Copy, Debug)]
pub struct Duration {
    pub divisions: u32,
    pub length: u32,
}

impl Duration {
    /// Creates a `Duration` from `divisions` and `length`
    ///
    /// Specify a duration by dividing a bar into N `divisions` and 
    /// by specifying a multiple of this 1/N division.
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mumuse::music::duration::Duration;
    /// let duration: Duration = Duration::new(16, 1);
    /// ```
    pub fn new(divisions: u32, length: u32) -> Self {
        Duration { divisions, length }
    }

    /// Convert duration into seconds
    ///
    /// In order to convert the duration into seconds, one need to declare a `bpm`
    /// (beats per minutes) and a `bpb` (beats per bar).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mumuse::music::duration::Duration;
    /// let duration: Duration = Duration::new(16, 1);
    /// let duration_seconds = duration.to_seconds(120.0, 4);
    /// ```
    pub fn to_seconds(&self, bpm: f64, bpb: u32) -> f64 {
        let bar_duration = (bpb as f64) * 60. / bpm;
        self.length as f64 * bar_duration / self.divisions as f64
    }
}

//! Time references

use crate::music::duration::Duration;
use num::integer::lcm;
use std::ops;

/// Time reference
#[derive(Clone, Copy, Debug)]
pub struct Time {
    /// starts at 1
    pub bar: u32,
    /// divide the bar in equal parts
    pub divisions: u32,
    /// nth division placement
    pub position: u32,
}

impl Time {
    /// Creates a new `Time` from a bar, divisions and position.
    ///
    /// A `Time` locates an event by dividing a given var in `divisions` equal pieces
    /// and by specifying a `position` for the event.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mumuse::music::time::Time;
    /// let t = Time::new(1, 4, 1);
    /// ```
    pub fn new(bar: u32, divisions: u32, position: u32) -> Self {
        Time {
            bar, 
            divisions, 
            position, 
        }
    }

    /// Converts a `Time` to seconds as `f64`.
    ///
    /// The conversion needs to have a given beats per minutes `bpm` value
    /// and the number of beats per bar `bpb` in order to compute time in seconds.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mumuse::music::time::Time;
    /// let t = Time::new(1, 4, 1);
    /// let t_s = t.to_seconds(120.0, 4);
    /// ```
    pub fn to_seconds(&self, bpm: f64, bpb: u32) -> f64 {
        let bar_duration = (bpb as f64) * 60. / bpm;
        bar_duration * ((self.bar - 1) as f64 + (self.position - 1) as f64 / self.divisions as f64)
    }
}

impl ops::Add<Duration> for Time {
    type Output = Time;
    /// Add `Duration` to `Time`
    ///
    /// The addition works by computing the least common multiple between
    /// `Duration` and `Time` divisions and computing the addition in the new divisions.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mumuse::music::time::Time;
    /// use mumuse::music::duration::Duration;
    /// let t = Time::new(1, 4, 1);
    /// let d = Duration::new(3, 1); // Triplet
    /// let new_t = t + d;
    /// ```
    fn add(self, rhs: Duration) -> Time {
        let new_div = lcm(self.divisions, rhs.divisions);
        let self_new_position: u32;

        // Positions start to 1 because we count beats as 1, 2, 3, ... until beats per nar.
        if self.position != 1 {
            self_new_position = self.position * new_div / self.divisions;
        } else {
            self_new_position = 1;
        }
        let mut new_pos = self_new_position + rhs.length * new_div / rhs.divisions;
        let mut new_bar = self.bar;

        // Wrap up of time when end of bar is reached
        if new_pos > new_div {
            new_bar += new_pos / new_div;
            new_pos %= new_div;
        }
        Time::new(new_bar, new_div, new_pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let time: Time = Time::new(1, 16, 1);
        assert_eq!(time.divisions, 16);
    }

    #[test]
    fn time_add_duration() {
        let time: Time = Time::new(1, 4, 1);
        let duration: Duration = Duration::new(4, 2);
        let sum = time + duration;
        assert_eq!(sum.divisions, 4);
        assert_eq!(sum.position, 3);
        assert_eq!(sum.bar, 1);
    }

    #[test]
    fn time_add_duration_from_middle() {
        let time: Time = Time::new(1, 4, 2);
        let duration: Duration = Duration::new(4, 1);
        let sum = time + duration;
        assert_eq!(sum.divisions, 4);
        assert_eq!(sum.position, 3);
        assert_eq!(sum.bar, 1);
    }

    #[test]
    fn time_add_duration_triplet() {
        let time: Time = Time::new(1, 4, 1);
        let duration: Duration = Duration::new(3, 1);
        let sum = time + duration;
        assert_eq!(sum.divisions, 12);
        assert_eq!(sum.position, 5);
        assert_eq!(sum.bar, 1);
    }

    #[test]
    fn time_add_duration_exceed() {
        let time: Time = Time::new(1, 4, 1);
        let duration: Duration = Duration::new(3, 3);
        let sum = time + duration;
        assert_eq!(sum.divisions, 12);
        assert_eq!(sum.position, 1);
        assert_eq!(sum.bar, 2);
    }
}

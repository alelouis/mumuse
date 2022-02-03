//! Time references

use crate::music::duration::Duration;
use num::integer::lcm;
use std::{fmt, ops};

/// Time reference
#[derive(Clone, Copy, Debug)]
pub struct Time {
    pub bar: u32,
    pub divisions: u32,
    pub position: u32,
}

impl Time {
    pub fn new(bar: u32, divisions: u32, position: u32) -> Self {
        Time {
            bar,
            divisions,
            position,
        }
    }

    // Convert time to seconds
    pub fn to_seconds(&self, bpm: f64, bpb: u32) -> f64 {
        let bar_duration = (bpb as f64) * 60. / bpm;
        bar_duration * ((self.bar - 1) as f64 + (self.position - 1) as f64 / self.divisions as f64)
    }
}

/// Overload operator + for Time + Duration
impl ops::Add<Duration> for Time {
    type Output = Time;
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
            new_pos = new_pos % new_div;
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

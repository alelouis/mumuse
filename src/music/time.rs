//! Time references

// TODO: Implement + operator for Time
// x.x.x.x is a shit notation actually, I need to find something better
// The most generic way I can think about would be to :
// Define measure, for example 2
// Define subdivisions of bar, for example 3 for triplet grid
// Define nth subdivision

// Examples : 
// third triplet in second bar
// Time(2, 3, 3)
// 
// fourth sixteenth note in first bar
// Time(1, 16, 4)
//
// second quintuplet note in third bar
// Time(3, 5, 2)
//
// Attach time to note ? Or convert time to seconds at eval time ?
// I don't know yet


/// Time signature
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct Meter {
    upper: u8,
    lower: u8,
}

/// Time reference
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct Time {
    meter: Meter,
    stamp: String,
    divisions: [u8; 4]
}

impl Meter {
    /// Constructor with time signature
    pub fn new(upper: u8, lower: u8) -> Self {
        Self { upper, lower }
    }
}

impl Time {
    /// Constructor with stamp, default Meter 4/4
    pub fn new(stamp: String) -> Self {
        let divisions = Self::parse_stamp(&stamp, &Meter::new(4, 4));
        Self {
            meter: Meter::new(4, 4),
            stamp,
            divisions
        }
    }

    /// Constructor with Meter
    pub fn with_meter(meter: Meter, stamp: String) -> Self {
        let divisions = Self::parse_stamp(&stamp, &meter);
        Self { 
            meter, 
            stamp,
            divisions
        }
    }

    /// Parse x.x.x.x String stamp into divisions array
    fn parse_stamp(stamp: &String, meter: &Meter) -> [u8; 4] {
        let mut r_div = stamp.split('.');
        let mut divisions = [0; 4];
        for div in 0..4 {
            divisions[div] = match r_div.next() {
                Some(n) => n.parse::<u8>().unwrap_or(1),
                None => 0
            }
        }
        divisions[1] = divisions[1]%(meter.upper);
        divisions[2] = divisions[2]%4;
        divisions[3] = divisions[3]%4;
        divisions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn autofill() {
        let time: Time = Time::new(String::from("1"));
        assert_eq!(time.divisions[0], 1);
        assert_eq!(time.divisions[1], 0);
        assert_eq!(time.divisions[2], 0);
        assert_eq!(time.divisions[3], 0);
    }
}
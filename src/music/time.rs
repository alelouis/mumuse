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

/// Time reference
#[derive(Clone, Debug)]
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let time: Time = Time::new(1, 16, 1);
        assert_eq!(time.divisions, 16);
    }
}

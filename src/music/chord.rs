//! Collection of Notes

use crate::music::common::Interval;
use crate::music::note::Note;
use itertools::Itertools;
use std::{fmt, ops};

/// A Chord contains a vector of Notes
#[derive(Debug, Default, Clone)]
pub struct Chord {
    pub notes: Vec<Note>,
}

impl Chord {
    /// Construct from Note vector
    pub fn new(notes: Vec<Note>) -> Self {
        Self { notes }
    }

    // Chord inversion
    pub fn invert(&self, inversion: usize) -> Self {
        let mut notes = self.notes.clone();
        let len = self.notes.len();
        for _ in 0..inversion {
            notes.rotate_left(1);
            notes[len - 1].octave += 1;
        }
        Self::new(notes)
    }
}

impl From<Vec<&str>> for Chord {
    fn from(notes: Vec<&str>) -> Self {
        Self::new(
            notes
                .iter()
                .filter_map(|note| Note::try_from(*note).ok())
                .collect(),
        )
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut notes: String = "".to_string();
        for (i, note) in (&self.notes).iter().enumerate() {
            notes += &format!("{}", note).to_string();
            if i != self.notes.len() - 1 {
                notes += ","
            }
        }
        write!(f, "Chord({})", notes)
    }
}

/// Overload operator + for Chord + Interval
impl ops::Add<Interval> for Chord {
    type Output = Chord;
    fn add(self, rhs: Interval) -> Chord {
        Chord::new(self.notes.into_iter().map(|n| n + rhs).collect_vec())
    }
}

/// Overload operator - for Chord - Interval
impl ops::Sub<Interval> for Chord {
    type Output = Chord;
    fn sub(self, rhs: Interval) -> Chord {
        Chord::new(self.notes.into_iter().map(|n| n - rhs).collect_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::music::common::Letter;

    /// Chord inversion
    #[test]
    fn inversion() {
        let chord = Chord::from(vec!["C0", "E1", "G2"]);
        let inversion = chord.invert(1);
        assert_eq!(inversion.notes[0].letter, Letter::E);
        assert_eq!(inversion.notes[1].letter, Letter::G);
        assert_eq!(inversion.notes[2].letter, Letter::C);
        assert_eq!(inversion.notes[2].octave, 1);
    }

    /// Chord transposition test with Interval
    #[test]
    fn transposition() {
        let chord = Chord::from(vec!["C0", "E1", "G2"]);
        let transposed = chord - Interval::Octave;
        assert_eq!(transposed.notes[0].octave, -1);
        assert_eq!(transposed.notes[1].octave, 0);
        assert_eq!(transposed.notes[2].octave, 1);
    }

    /// Chord creation from string
    #[test]
    fn from_str() {
        let chord = Chord::from(vec!["C0", "E1", "G2", "U2"]); // unknown is ignored
        assert_eq!(chord.notes[0].letter, Letter::C);
        assert_eq!(chord.notes[1].letter, Letter::E);
        assert_eq!(chord.notes[2].letter, Letter::G);
    }
}

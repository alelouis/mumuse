//! Root note and a set of Intervals

use crate::music::common::Interval;
use crate::music::common::Interval::*;
use crate::music::note::Note;

/// A scale consists in a root Note and a vector of Intervals
pub struct Scale {
    pub root: Note,
    pub intervals: Vec<Interval>,
}

impl Scale {
    /// Default constructor
    pub fn new(root: Note, intervals: Vec<Interval>) -> Self {
        Scale { root, intervals }
    }

    /// Major scale intervals
    pub const MAJOR: [Interval; 6] = [
        MajorSecond,
        MajorThird,
        Fourth,
        Fifth,
        MajorSixth,
        MajorSeventh,
    ];
    /// Minor (natural) scale intervals
    pub const MINOR: [Interval; 6] = [
        MajorSecond,
        MinorThird,
        Fourth,
        Fifth,
        MinorSixth,
        MinorSeventh,
    ];
    /// Minor (harmonic) scale intervals
    pub const MINOR_HARMONIC: [Interval; 6] = [
        MajorSecond,
        MinorThird,
        Fourth,
        Fifth,
        MinorSixth,
        MajorSeventh,
    ];

    /// Get major scale from root Note
    pub fn major(root: Note) -> Self {
        Self::new(root, Self::MAJOR.to_vec())
    }

    /// Get minor (natural) scale from root Note
    pub fn minor(root: Note) -> Self {
        Self::new(root, Self::MINOR.to_vec())
    }

    /// Get minor (harmonic) scale from root Note
    pub fn minor_harmonic(root: Note) -> Self {
        Self::new(root, Self::MINOR_HARMONIC.to_vec())
    }

    /// Get Note vector from Scale
    pub fn notes(&self) -> Vec<Note> {
        let mut out: Vec<Note> = vec![self.root];
        for interval in &self.intervals {
            out.push(self.root + *interval)
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::music::common::Letter;

    #[test]
    fn get_notes() {
        let root = Note::from_str("C0").unwrap();
        let intervals = vec![MajorSecond, Fifth];
        let scale = Scale::new(root, intervals);
        let notes = scale.notes();
        assert_eq!(notes[0].letter, Letter::C);
        assert_eq!(notes[1].letter, Letter::D);
    }

    #[test]
    fn major() {
        let root = Note::from_str("C0").unwrap();
        let major_scale = Scale::major(root);
        assert_eq!(major_scale.notes()[0].letter, Letter::C);
        assert_eq!(major_scale.notes()[1].letter, Letter::D);
        assert_eq!(major_scale.notes()[2].letter, Letter::E);
        assert_eq!(major_scale.notes()[3].letter, Letter::F);
        assert_eq!(major_scale.notes()[4].letter, Letter::G);
        assert_eq!(major_scale.notes()[5].letter, Letter::A);
        assert_eq!(major_scale.notes()[6].letter, Letter::B)
    }

    #[test]
    fn minor() {
        let root = Note::from_str("A0").unwrap();
        let major_scale = Scale::minor(root);
        assert_eq!(major_scale.notes()[0].letter, Letter::A);
        assert_eq!(major_scale.notes()[1].letter, Letter::B);
        assert_eq!(major_scale.notes()[2].letter, Letter::C);
        assert_eq!(major_scale.notes()[2].octave, 1);
        assert_eq!(major_scale.notes()[3].letter, Letter::D);
        assert_eq!(major_scale.notes()[4].letter, Letter::E);
        assert_eq!(major_scale.notes()[5].letter, Letter::F);
        assert_eq!(major_scale.notes()[6].letter, Letter::G)
    }
}

//! Root note and a set of Intervals

use itertools::Itertools;

use crate::music::chord::Chord;
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
    pub const MAJOR: [Interval; 7] = [
        Unison,
        MajorSecond,
        MajorThird,
        Fourth,
        Fifth,
        MajorSixth,
        MajorSeventh,
    ];
    /// Minor (natural) scale intervals
    pub const MINOR: [Interval; 7] = [
        Unison,
        MajorSecond,
        MinorThird,
        Fourth,
        Fifth,
        MinorSixth,
        MinorSeventh,
    ];
    /// Minor (harmonic) scale intervals
    pub const MINOR_HARMONIC: [Interval; 7] = [
        Unison,
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

    /// One chord built by thirds (if scale built by thirds).
    pub fn one(&self) -> Chord {
        Chord::new(self.build_by_steps(0, 2, 3))
    }

    /// Two chord built by thirds (if scale built by thirds).
    pub fn two(&self) -> Chord {
        Chord::new(self.build_by_steps(1, 2, 3))
    }

    /// Three chord built by thirds (if scale built by thirds).
    pub fn three(&self) -> Chord {
        Chord::new(self.build_by_steps(2, 2, 3))
    }

    /// Four chord built by thirds (if scale built by thirds).
    pub fn four(&self) -> Chord {
        Chord::new(self.build_by_steps(3, 2, 3))
    }

    /// Five chord built by thirds (if scale built by thirds).
    pub fn five(&self) -> Chord {
        Chord::new(self.build_by_steps(4, 2, 3))
    }

    /// Six chord built by thirds (if scale built by thirds).
    pub fn six(&self) -> Chord {
        Chord::new(self.build_by_steps(5, 2, 3))
    }

    /// Seven chord built by thirds (if scale built by thirds).
    pub fn seven(&self) -> Chord {
        Chord::new(self.build_by_steps(6, 2, 3))
    }

    pub fn build_by_steps(&self, root: usize, step: usize, length: usize) -> Vec<Note> {
        self.intervals
            .clone()
            .into_iter()
            .cycle()
            .skip(root)
            .step_by(step)
            .map(|n| self.root + n)
            .take(length)
            .collect_vec()
    }

    /// Get Note vector from Scale
    pub fn notes(&self) -> Vec<Note> {
        let mut out: Vec<Note> = vec![];
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
        let root = Note::try_from("C0").unwrap();
        let intervals = vec![Unison, MajorSecond];
        let scale = Scale::new(root, intervals);
        let notes = scale.notes();
        assert_eq!(notes[0].letter, Letter::C);
        assert_eq!(notes[1].letter, Letter::D);
    }

    #[test]
    fn get_one() {
        let root = Note::try_from("C0").unwrap();
        let major_scale = Scale::major(root);
        let one_chord = major_scale.one();
        println!("{}", one_chord);
        assert_eq!(one_chord.notes[0].letter, Letter::C);
        assert_eq!(one_chord.notes[1].letter, Letter::E);
        assert_eq!(one_chord.notes[2].letter, Letter::G);
    }

    #[test]
    fn get_two() {
        let root = Note::try_from("C0").unwrap();
        let major_scale = Scale::major(root);
        let two_chord = major_scale.two();
        println!("{}", two_chord);
        assert_eq!(two_chord.notes[0].letter, Letter::D);
        assert_eq!(two_chord.notes[1].letter, Letter::F);
        assert_eq!(two_chord.notes[2].letter, Letter::A);
    }

    #[test]
    fn major() {
        let root = Note::try_from("C0").unwrap();
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
        let root = Note::try_from("A0").unwrap();
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

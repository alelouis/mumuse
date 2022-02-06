//! Root note and a set of Intervals

use itertools::Itertools;
use std::fmt;

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

    /// Get mode n of current scale.
    ///
    /// The mode specifies the starting point of the sequence
    /// of intervals defining the scale.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mumuse::music::scale::Scale;
    /// use mumuse::music::note::Note;
    /// let root = Note::try_from("C0").unwrap();
    /// let major_scale = Scale::major(root);
    /// let ionian = major_scale.mode(1); 
    /// ```
    pub fn mode(&self, n: i8) -> Self {
        let mut mode = n as i8 - 1;
        while mode < 0 {
            // wrap around invalid modes identifiers
            mode += self.intervals.len() as i8;
        }

        // Convert intervals to int for computations
        let reference = self.intervals.clone()[mode as usize] as i8;
        let mut int_intervals = self
            .intervals
            .clone()
            .iter()
            .map(|x| {
                let mut a = *x as i8 - reference;
                if a < 0 {
                    a += 12;
                }
                a
            })
            .collect_vec();

        // Place origin of mode in first place (which will be Unison)
        int_intervals.rotate_left(mode as usize);

        // Convert back to intervals
        let new_intervals: Vec<Interval> = int_intervals
            .into_iter()
            .filter_map(|i| num::FromPrimitive::from_u32(i as u32))
            .collect_vec();

        Self::new(self.root, new_intervals)
    }

    /// Get major scale from root Note
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mumuse::music::scale::Scale;
    /// use mumuse::music::note::Note;
    /// let root = Note::try_from("C0").unwrap();
    /// let major_scale = Scale::major(root);
    /// ```
    pub fn major(root: Note) -> Self {
        Self::new(root, Self::MAJOR.to_vec())
    }

    /// Get minor (natural) scale from root Note
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mumuse::music::scale::Scale;
    /// use mumuse::music::note::Note;
    /// let root = Note::try_from("C0").unwrap();
    /// let minor_scale = Scale::minor(root);
    /// ```
    pub fn minor(root: Note) -> Self {
        Self::new(root, Self::MINOR.to_vec())
    }

    /// get minor (harmonic) scale from root note
    ///
    /// # examples
    ///
    /// basic usage:
    ///
    /// ```
    /// use mumuse::music::scale::Scale;
    /// use mumuse::music::note::Note;
    /// let root = Note::try_from("C4").unwrap();
    /// let minor_harmonic_scale = Scale::minor_harmonic(root);
    /// ```
    pub fn minor_harmonic(root: Note) -> Self {
        Self::new(root, Self::MINOR_HARMONIC.to_vec())
    }

    /// Build chord by degrees (degree = 1 same as self.one() call)
    ///
    /// Specify the number of thirds to stack (scale should be built by thirds).
    ///
    /// # examples
    ///
    /// basic usage:
    ///
    /// ```
    /// use mumuse::music::scale::Scale;
    /// use mumuse::music::note::Note;
    /// let root = Note::try_from("A3").unwrap();
    /// let five = Scale::major(root).by_degree(5, 3);
    /// ```
    pub fn by_degree(&self, degree: usize, len: usize) -> Chord {
        Chord::new(self.build_by_steps(degree - 1, 2, len))
    }

    /// One chord built by thirds (if scale built by thirds).
    ///
    /// # examples
    ///
    /// basic usage:
    ///
    /// ```
    /// use mumuse::music::scale::Scale;
    /// use mumuse::music::note::Note;
    /// let root = Note::try_from("A3").unwrap();
    /// let one = Scale::major(root).one(3);
    /// ```
    pub fn one(&self, len: usize) -> Chord {
        Chord::new(self.build_by_steps(0, 2, len))
    }

    /// Two chord built by thirds (if scale built by thirds).
    ///
    /// # examples
    ///
    /// basic usage:
    ///
    /// ```
    /// use mumuse::music::scale::Scale;
    /// use mumuse::music::note::Note;
    /// let root = Note::try_from("A3").unwrap();
    /// let two = Scale::major(root).two(3);
    /// ```
    pub fn two(&self, len: usize) -> Chord {
        Chord::new(self.build_by_steps(1, 2, len))
    }

    /// Three chord built by thirds (if scale built by thirds).
    ///
    /// # examples
    ///
    /// basic usage:
    ///
    /// ```
    /// use mumuse::music::scale::Scale;
    /// use mumuse::music::note::Note;
    /// let root = Note::try_from("A3").unwrap();
    /// let three = Scale::major(root).three(3);
    /// ```
    pub fn three(&self, len: usize) -> Chord {
        Chord::new(self.build_by_steps(2, 2, len))
    }

    /// Four chord built by thirds (if scale built by thirds).
    ///
    /// # examples
    ///
    /// basic usage:
    ///
    /// ```
    /// use mumuse::music::scale::Scale;
    /// use mumuse::music::note::Note;
    /// let root = Note::try_from("A3").unwrap();
    /// let four = Scale::major(root).four(3);
    /// ```
    pub fn four(&self, len: usize) -> Chord {
        Chord::new(self.build_by_steps(3, 2, len))
    }

    /// Five chord built by thirds (if scale built by thirds).
    ///
    /// # examples
    ///
    /// basic usage:
    ///
    /// ```
    /// use mumuse::music::scale::Scale;
    /// use mumuse::music::note::Note;
    /// let root = Note::try_from("A3").unwrap();
    /// let five = Scale::major(root).five(3);
    /// ```
    pub fn five(&self, len: usize) -> Chord {
        Chord::new(self.build_by_steps(4, 2, len))
    }

    /// Six chord built by thirds (if scale built by thirds).
    ///
    /// # examples
    ///
    /// basic usage:
    ///
    /// ```
    /// use mumuse::music::scale::Scale;
    /// use mumuse::music::note::Note;
    /// let root = Note::try_from("A3").unwrap();
    /// let six = Scale::major(root).six(3);
    /// ```
    pub fn six(&self, len: usize) -> Chord {
        Chord::new(self.build_by_steps(5, 2, len))
    }

    /// Seven chord built by thirds (if scale built by thirds).
    ///
    /// # examples
    ///
    /// basic usage:
    ///
    /// ```
    /// use mumuse::music::scale::Scale;
    /// use mumuse::music::note::Note;
    /// let root = Note::try_from("A3").unwrap();
    /// let seven = Scale::major(root).seven(3);
    /// ```
    pub fn seven(&self, len: usize) -> Chord {
        Chord::new(self.build_by_steps(6, 2, len))
    }

    /// Build notes of scale from intervals and steps
    fn build_by_steps(&self, root: usize, step: usize, length: usize) -> Vec<Note> {
        // We add all intervals of the scale one octave higher for chord creation
        let mut intervals_octave_up: Vec<Interval> = self
            .intervals
            .clone()
            .into_iter()
            .filter_map(|i| num::FromPrimitive::from_u32(i as u32 + 12))
            .collect_vec();

        // Concatenate all intervals
        let mut intervals = self.intervals.clone();
        intervals.append(&mut intervals_octave_up);

        // Step in interval vector
        intervals
            .clone()
            .into_iter()
            .cycle() // Wraps around if step / length combination exceeds two octave span
            .skip(root)
            .step_by(step)
            .map(|n| self.root + n)
            .take(length)
            .collect_vec()
    }

    /// Get `Note` vector from Scale
    ///
    /// # examples
    ///
    /// basic usage:
    ///
    /// ```
    /// use mumuse::music::scale::Scale;
    /// use mumuse::music::note::Note;
    /// let root = Note::try_from("A3").unwrap();
    /// let scale = Scale::major(root);
    /// let notes = scale.notes();
    /// ```
    pub fn notes(&self) -> Vec<Note> {
        self.intervals
            .clone()
            .into_iter()
            .map(|interval| self.root + interval)
            .collect_vec()
    }
}

/// Display trait for Scale
impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut notes: String = "".to_string();
        for (i, note) in (&self.notes()).iter().enumerate() {
            notes += &format!("{}", note).to_string();
            if i != self.notes().len() - 1 {
                notes += ","
            }
        }
        write!(f, "Scale({})", notes)
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
        let one_chord = major_scale.one(3);
        assert_eq!(one_chord.notes[0].letter, Letter::C);
        assert_eq!(one_chord.notes[1].letter, Letter::E);
        assert_eq!(one_chord.notes[2].letter, Letter::G);
    }

    #[test]
    fn get_two() {
        let root = Note::try_from("C0").unwrap();
        let major_scale = Scale::major(root);
        let two_chord = major_scale.two(3);
        assert_eq!(two_chord.notes[0].letter, Letter::D);
        assert_eq!(two_chord.notes[1].letter, Letter::F);
        assert_eq!(two_chord.notes[2].letter, Letter::A);
    }

    #[test]
    fn major() {
        let root = Note::try_from("C0").unwrap();
        let major_scale = Scale::major(root);
        let c_major_scale = [
            Letter::C,
            Letter::D,
            Letter::E,
            Letter::F,
            Letter::G,
            Letter::A,
            Letter::B,
        ];
        for i in 0..7 {
            assert_eq!(major_scale.notes()[i as usize].letter, c_major_scale[i]);
        }
    }

    #[test]
    fn minor() {
        let root = Note::try_from("A0").unwrap();
        let minor_scale = Scale::minor(root);
        let a_minor_scale = [
            Letter::A,
            Letter::B,
            Letter::C,
            Letter::D,
            Letter::E,
            Letter::F,
            Letter::G,
        ];
        for i in 0..7 {
            assert_eq!(minor_scale.notes()[i as usize].letter, a_minor_scale[i]);
        }
    }

    #[test]
    fn modes() {
        let root = Note::try_from("C0").unwrap();
        let dorian_scale = Scale::major(root).mode(2);
        let c_dorian_scale = [
            Letter::C,
            Letter::D,
            Letter::Eb,
            Letter::F,
            Letter::G,
            Letter::A,
            Letter::Bb,
        ];
        for i in 0..7 {
            assert_eq!(dorian_scale.notes()[i as usize].letter, c_dorian_scale[i]);
        }
    }
}

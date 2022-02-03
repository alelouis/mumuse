//! Collection of Notes

use crate::music::common::Interval;
use crate::music::note::Note;
use itertools::Itertools;
use std::{fmt, ops};

/// Chord is a vector a Notes
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

    // TODO pub fn intervals()

    /// Finds optimal minimum movement chord to target
    pub fn voicelead_to(&self, target: &Self) -> Option<Self> {
        let mut dist_vec: Vec<Vec<Vec<u8>>> = vec![];
        let mut max: u32 = 100;
        let mut voice_lead: Option<Chord> = None;
        let chord_len: usize = 4;

        // Computing distance vector between two chords
        for note in &self.notes {
            let mut note_vec: Vec<Vec<u8>> = vec![];
            for other_note in &target.notes {
                let mut octave_vec: Vec<u8> = vec![];

                // Octave span should be equal to chord length
                for octave in other_note.octave - 1..=other_note.octave + 2 {
                    let swipe_note = Note::new(other_note.letter, octave);
                    octave_vec.push(note.dist_to(&swipe_note));
                }
                note_vec.push(octave_vec);
            }
            dist_vec.push(note_vec);
        }

        // Finding minimal movement cost chord
        for p in (0..chord_len).permutations(chord_len) {
            for c in (0..chord_len).combinations_with_replacement(chord_len) {
                let mut sum: u32 = 0;
                for n in 0..chord_len {
                    sum += dist_vec[n][p[n]][c[n]] as u32;
                }
                if sum < max {
                    max = sum;
                    let mut note_vec: Vec<Note> = vec![];
                    for note in 0..chord_len {
                        note_vec.push(Note::new(
                            target.notes[p[note]].letter,
                            target.notes[p[note]].octave + c[note] as i8 - 1,
                        ));
                    }
                    voice_lead = Some(Chord::new(note_vec));
                }
            }
        }
        voice_lead
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

    /// Chord optimal voice leading
    #[test]
    fn transition() {
        let from = Chord::from(vec!["C4", "E4", "G4", "B4"]);
        let target = Chord::from(vec!["E4", "G4", "B4", "D5"]);
        let voiceleaded = from.voicelead_to(&target).unwrap();
        assert_eq!(voiceleaded.notes.len(), target.notes.len());
    }
}

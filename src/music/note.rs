//! Letter and octave

use crate::messages::Data;
use crate::music::common::{find_letter_idx, Interval, Letter, KEYBOARD};
use std::{fmt, ops};

/// Note abstraction with letter and octave
#[derive(Debug, Clone, Copy, Default)]
pub struct Note {
    pub letter: Letter,
    pub octave: i8,
}

impl Note {
    /// Construct Note from Letter and octave
    pub fn new(letter: Letter, octave: i8) -> Self {
        Note { letter, octave }
    }

    /// Compute distance in semitones between two notes
    pub fn dist_to(&self, other: &Note) -> u8 {
        let octave_diff: i8 = self.octave as i8 - other.octave as i8;
        (find_letter_idx(self.letter) - find_letter_idx(other.letter) + octave_diff * 12)
            .abs()
            .try_into()
            .unwrap()
    }
}

/// Display trait for Note
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Note({:?}{})", self.letter, self.octave)
    }
}

/// Conversion from Data::KeyNumber
impl TryFrom<&Data> for Note {
    type Error = ();
    fn try_from(kn: &Data) -> Result<Self, Self::Error> {
        match kn {
            Data::KeyNumber(x) => {
                let index: usize = ((x - 21) % 12) as usize;
                Ok(Note::new(KEYBOARD[index], (*x as i8 - 21) / 12))
            }
            _ => Err(()),
        }
    }
}

/// Conversion from str
impl TryFrom<&str> for Note {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let octave_str: &i8 = &s[s.len() - 1..].parse::<i8>().unwrap();
        let letter: Option<Letter> = match &s[0..s.len() - 1] {
            "C" | "B#" => Some(Letter::C),
            "Db" | "C#" => Some(Letter::Db),
            "D" => Some(Letter::D),
            "Eb" | "D#" => Some(Letter::Eb),
            "E" | "Fb" => Some(Letter::E),
            "F" | "E#" => Some(Letter::F),
            "Gb" | "F#" => Some(Letter::Gb),
            "G" => Some(Letter::G),
            "Ab" | "G#" => Some(Letter::Ab),
            "A" => Some(Letter::A),
            "Bb" | "A#" => Some(Letter::Bb),
            "B" | "Cb" => Some(Letter::B),
            _ => None,
        };
        match letter {
            Some(l) => Ok(Note::new(l, *octave_str)),
            None => Err(()),
        }
    }
}

/// Overload operator + for Note + Interval
impl ops::Add<Interval> for Note {
    type Output = Note;
    fn add(self, rhs: Interval) -> Note {
        let self_index = find_letter_idx(self.letter);
        let target_index = self_index + rhs as i8;
        Note::new(
            KEYBOARD[(target_index % 12) as usize],
            self.octave + (target_index / 12) as i8,
        )
    }
}

/// Overload operator - for Note - Interval
impl ops::Sub<Interval> for Note {
    type Output = Note;
    fn sub(self, rhs: Interval) -> Note {
        let self_index = find_letter_idx(self.letter);
        let mut target_index: i8 = self_index - (rhs as i8);
        if target_index < 0 {
            target_index += 12 * (1 - target_index / 12)
        }
        Note::new(
            KEYBOARD[(target_index % 12) as usize],
            self.octave + (self_index - (rhs as i8)) / 12,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_from_str() {
        let a = Note::try_from("A0").unwrap();
        assert_eq!(a.letter, Letter::A);
        assert_eq!(a.octave, 0);

        let bb = Note::try_from("Bb2").unwrap();
        assert_eq!(bb.letter, Letter::Bb);
        assert_eq!(bb.octave, 2);
    }

    #[test]
    fn note_add_interval() {
        let c = Note::try_from("C2").unwrap();
        for i in 0..24 {
            let interval: Interval = num::FromPrimitive::from_u32(i).unwrap();
            let letter: Letter = num::FromPrimitive::from_u32(i % 12).unwrap();
            let note = c + interval;
            assert_eq!(note.letter, letter);
            assert_eq!(note.octave as i8, 2 + (i as i8) / 12);
        }
    }

    #[test]
    fn note_sub_interval() {
        let c = Note::try_from("C2").unwrap();
        for i in 0..24 {
            let interval: Interval = num::FromPrimitive::from_u32(i).unwrap();
            let letter: Letter = num::FromPrimitive::from_u32((24 - i) % 12).unwrap();
            let note = c - interval;
            assert_eq!(note.letter, letter);
            assert_eq!(note.octave as i8, 2 - (i as i8) / 12);
        }
    }

    #[test]
    fn distance_between_notes() {
        let note_1 = Note {
            letter: Letter::C,
            octave: 0,
        };
        let note_2 = Note {
            letter: Letter::E,
            octave: 0,
        };
        let note_3 = Note {
            letter: Letter::E,
            octave: 1,
        };
        let note_4 = Note {
            letter: Letter::B,
            octave: 0,
        };
        let note_5 = Note {
            letter: Letter::C,
            octave: 1,
        };
        assert_eq!(note_1.dist_to(&note_2), 4);
        assert_eq!(note_2.dist_to(&note_1), 4);
        assert_eq!(note_1.dist_to(&note_3), 16);
        assert_eq!(note_3.dist_to(&note_1), 16);
        assert_eq!(note_4.dist_to(&note_5), 1);
    }
}

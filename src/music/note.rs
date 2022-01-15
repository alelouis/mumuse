//! Letter and octave

use crate::messages::Data;
use crate::music::common::Interval;
use crate::music::common::Letter;
use crate::music::common::KEYBOARD;
use std::fmt;
use std::ops;

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

    /// Construct Note from midi key number
    pub fn from_key_number(kn: &Data) -> Option<Self> {
        match kn {
            Data::KeyNumber(x) => {
                let index: usize = ((x - 21) % 12) as usize;
                Some(Note {
                    letter: KEYBOARD[index],
                    octave: (*x as i8 - 21) / 12,
                })
            }
            _ => None,
        }
    }

    /// Convert Note to midi key number
    pub fn to_key_number(&self) -> Data {
        let p = KEYBOARD.iter().position(|&n| n == self.letter).unwrap() as u8;
        Data::KeyNumber(12 + p + (self.octave as u8) * 12)
    }

    /// Compute distance in semitones between two notes
    pub fn dist_to(&self, other: &Note) -> u8 {
        let octave_difference: i8 = self.octave as i8 - other.octave as i8;
        let self_index: i8 = KEYBOARD.iter().position(|&x| x == self.letter).unwrap() as i8;
        let other_index: i8 = KEYBOARD.iter().position(|&x| x == other.letter).unwrap() as i8;
        (self_index - other_index + octave_difference * 12)
            .abs()
            .try_into()
            .unwrap()
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{}", self.letter, self.octave)
    }
}

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

// Overload operator + for Note + Interval
impl ops::Add<Interval> for Note {
    type Output = Note;
    fn add(self, rhs: Interval) -> Note {
        let self_index: u8 = KEYBOARD.iter().position(|&x| x == self.letter).unwrap() as u8;
        let target_index: u8 = self_index + rhs as u8;
        Note::new(
            KEYBOARD[(target_index % 12) as usize],
            self.octave + (target_index / 12) as i8,
        )
    }
}

// Overload operator - for Note - Interval
impl ops::Sub<Interval> for Note {
    type Output = Note;
    fn sub(self, rhs: Interval) -> Note {
        let self_index: i8 = KEYBOARD.iter().position(|&x| x == self.letter).unwrap() as i8;
        let mut target_index: i8 = self_index - (rhs as i8);
        if target_index < 0 {
            target_index += 12
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
        let c = Note::try_from("C0").unwrap();
        assert_eq!((c + Interval::Unison).letter, Letter::C);
        assert_eq!((c + Interval::MinorSecond).letter, Letter::Db);
        assert_eq!((c + Interval::MajorSecond).letter, Letter::D);
        assert_eq!((c + Interval::MinorThird).letter, Letter::Eb);
        assert_eq!((c + Interval::MajorThird).letter, Letter::E);
        assert_eq!((c + Interval::Fourth).letter, Letter::F);
        assert_eq!((c + Interval::Tritone).letter, Letter::Gb);
        assert_eq!((c + Interval::Fifth).letter, Letter::G);
        assert_eq!((c + Interval::MinorSixth).letter, Letter::Ab);
        assert_eq!((c + Interval::MajorSixth).letter, Letter::A);
        assert_eq!((c + Interval::MinorSeventh).letter, Letter::Bb);
        assert_eq!((c + Interval::MajorSeventh).letter, Letter::B);
        assert_eq!((c + Interval::Octave).letter, Letter::C);
        assert_eq!((c + Interval::Octave).octave, 1);
    }

    #[test]
    fn note_sub_interval() {
        let c = Note::try_from("C1").unwrap();
        assert_eq!((c - Interval::Octave).octave, 0);
        assert_eq!((c - Interval::Octave).letter, Letter::C);
        assert_eq!((c - Interval::MajorSeventh).letter, Letter::Db);
        assert_eq!((c - Interval::MinorSeventh).letter, Letter::D);
        assert_eq!((c - Interval::MajorSixth).letter, Letter::Eb);
        assert_eq!((c - Interval::MinorSixth).letter, Letter::E);
        assert_eq!((c - Interval::Fifth).letter, Letter::F);
        assert_eq!((c - Interval::Tritone).letter, Letter::Gb);
        assert_eq!((c - Interval::Fourth).letter, Letter::G);
        assert_eq!((c - Interval::MajorThird).letter, Letter::Ab);
        assert_eq!((c - Interval::MinorThird).letter, Letter::A);
        assert_eq!((c - Interval::MajorSecond).letter, Letter::Bb);
        assert_eq!((c - Interval::MinorSecond).letter, Letter::B);
        assert_eq!((c - Interval::Unison).letter, Letter::C);
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

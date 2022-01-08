use crate::messages::Data;
use crate::music::common::Letter;
use crate::music::common::KEYBOARD;
use std::fmt;

// Note abstraction with letter and octave
#[derive(Debug)]
pub struct Note {
    pub letter: Letter,
    pub octave: u8,
}

impl Note {
    // Find letter and octave from midi key number (Data::KeyNumber)
    pub fn new(letter: Letter, octave: u8) -> Self {
        Note { letter, octave }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let letter_str: &str = &s[0..s.len() - 1];
        let octave_str: &str = &s[s.len() - 1..];
        let letter: Option<Letter> = match letter_str {
            "C" => Some(Letter::C),
            "Db" => Some(Letter::Db),
            "D" => Some(Letter::D),
            "Eb" => Some(Letter::Eb),
            "E" => Some(Letter::E),
            "F" => Some(Letter::F),
            "Gb" => Some(Letter::Gb),
            "G" => Some(Letter::G),
            "Ab" => Some(Letter::Ab),
            "A" => Some(Letter::A),
            "Bb" => Some(Letter::Bb),
            "B" => Some(Letter::B),
            _ => None,
        };
        match letter {
            Some(l) => Some(Note::new(l, octave_str.parse::<u8>().unwrap())),
            None => None,
        }
    }

    pub fn from_key_number(kn: &Data) -> Option<Self> {
        match kn {
            Data::KeyNumber(x) => {
                let index: usize = ((x - 21) % 12) as usize;
                Some(Note {
                    letter: KEYBOARD[index],
                    octave: (x - 21) / 12,
                })
            }
            _ => None,
        }
    }

    pub fn to_key_number(&self) -> u8 {
        let p = KEYBOARD.iter().position(|&n| n == self.letter).unwrap() as u8;
        12 + p + self.octave * 12
    }

    // Compute distance in semitones between two notes
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

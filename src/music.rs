use crate::messages::Data;
use std::fmt;

// Note abstraction with letter and octave
#[derive(Debug)]
pub struct Note {
    pub letter: String,
    pub octave: u8,
}

impl Note {
    // Find letter and octave from midi key number (Data::KeyNumber)
    pub fn from_key_number(kn: &Data) -> Option<Self> {
        let note_names = [
            "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
        ];
        match kn {
            Data::KeyNumber(x) => {
                let index: usize = ((x - 21) % 12) as usize;
                Some(Note {
                    letter: note_names[index].to_string(),
                    octave: (x - 21) / 12,
                })
            }
            _ => None,
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.letter, self.octave)
    }
}

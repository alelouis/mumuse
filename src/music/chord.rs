use crate::music::note::Note;
use itertools::Itertools;
use std::fmt;

// Chord abstraction
#[derive(Debug)]
pub struct Chord {
    pub notes: Vec<Note>,
}


impl Chord {
    // Construct from Note vector
    pub fn new(notes: Vec<Note>) -> Self {
        Self { notes }
    }

    // Construct chord from vector str slices
    pub fn from_str(notes: Vec<&str>) -> Self {
        Self::new(
            notes
                .iter()
                .map(|note| Note::from_str(note).unwrap())
                .collect(),
        )
    }

    // Finds optimal minimum movement chord to target
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
                            target.notes[p[note]].octave + c[note] as u8 - 1,
                        ));
                    }
                    voice_lead = Some(Chord::new(note_vec));
                }
            }
        }
        voice_lead
        }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut notes: String = "".to_string();
        for (i, note) in (&self.notes).iter().enumerate() {
            notes += &format!("{}", note).to_string();
            if i != self.notes.len()-1 {
                notes += ","
            }
        }
        write!(f, "Chord({})", notes)
    }
}
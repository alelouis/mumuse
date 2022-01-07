use crate::messages;
use crate::music;
use crate::music::{Chord, Letter, Note};

// Tests conversion from Midi to Note
#[test]
fn from_key_number_to_note() {
    for kn in 21..127 {
        let data_kn = messages::Data::KeyNumber(kn);
        let note = match Note::from_key_number(&data_kn) {
            Some(note) => note,
            None => panic!("Oups"),
        };
        assert_eq!(note.letter, music::KEYBOARD[((kn - 21) % 12) as usize]);
        assert_eq!(note.octave, (kn - 21) / 12);
    }
}

// Note creation from string
#[test]
fn note_from_str() {
    let a = Note::from_str("A0").unwrap();
    assert_eq!(a.letter, Letter::A);
    assert_eq!(a.octave, 0);

    let bb = Note::from_str("Bb2").unwrap();
    assert_eq!(bb.letter, Letter::Bb);
    assert_eq!(bb.octave, 2);
}

// Distances in semitones between two Notes
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

// Chord creation from struct
#[test]
fn chord_from_struct() {
    let note_1 = Note {
        letter: Letter::C,
        octave: 0,
    };
    let note_2 = Note {
        letter: Letter::E,
        octave: 0,
    };
    let note_3 = Note {
        letter: Letter::G,
        octave: 0,
    };
    let chord = Chord::new(vec![note_1, note_2, note_3]);
    assert_eq!(chord.notes.len(), 3);
}

// Chord creation from string
#[test]
fn chord_from_str() {
    let chord = Chord::from_str(vec!["C0", "E1", "G2"]);
    assert_eq!(chord.notes[0].letter, Letter::C);
    assert_eq!(chord.notes[1].letter, Letter::E);
    assert_eq!(chord.notes[2].letter, Letter::G);
}

// Chord optimal voice leading
#[test]
fn chord_transition() {
    let from = Chord::from_str(vec!["C4", "E4", "G4", "B4"]);
    let target = Chord::from_str(vec!["E4", "G4", "B4", "D5"]);
    let voiceleaded = from.voicelead_to(&target).unwrap();
    assert_eq!(voiceleaded.notes.len(), target.notes.len());
    println!("{}", target);
    println!("{}", voiceleaded);
}

use crate::messages;
use crate::music;

// Tests conversion from Midi to Note
#[test]
fn from_key_number_to_note() {
    for kn in 21..127 {
        let data_kn = messages::Data::KeyNumber(kn);
        let note = match music::Note::from_key_number(&data_kn) {
            Some(note) => note,
            None => panic!("Oups"),
        };
        assert_eq!(note.letter, music::KEYBOARD[((kn - 21) % 12) as usize]);
        assert_eq!(note.octave, (kn - 21) / 12);
    }
}

#[test]
fn distance_between_notes() {
    let note_1 = music::Note {
        letter: music::Letter::C,
        octave: 0,
    };
    let note_2 = music::Note {
        letter: music::Letter::E,
        octave: 0,
    };
    let note_3 = music::Note {
        letter: music::Letter::E,
        octave: 1,
    };
    let note_4 = music::Note {
        letter: music::Letter::B,
        octave: 0,
    };
    let note_5 = music::Note {
        letter: music::Letter::C,
        octave: 1,
    };
    assert_eq!(note_1.dist_to(&note_2), 4);
    assert_eq!(note_2.dist_to(&note_1), 4);
    assert_eq!(note_1.dist_to(&note_3), 16);
    assert_eq!(note_3.dist_to(&note_1), 16);
    assert_eq!(note_4.dist_to(&note_5), 1);
}

#[test]
fn create_chord() {
    let note_1 = music::Note {
        letter: music::Letter::C,
        octave: 0,
    };
    let note_2 = music::Note {
        letter: music::Letter::E,
        octave: 0,
    };
    let note_3 = music::Note {
        letter: music::Letter::G,
        octave: 0,
    };
    let chord = music::Chord::new(vec![note_1, note_2, note_3]);
    assert_eq!(chord.notes.len(), 3);
}

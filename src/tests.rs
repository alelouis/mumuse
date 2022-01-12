//! Integration tests

/// Tests conversion from Midi to Note
#[test]
fn from_key_number_to_note() {
    use crate::messages;
    use crate::music::common::KEYBOARD;
    use crate::music::note::Note;
    
    for kn in 21..127 {
        let data_kn = messages::Data::KeyNumber(kn);
        let note = match Note::from_key_number(&data_kn) {
            Some(note) => note,
            None => panic!("Keynumber invalid."),
        };
        assert_eq!(note.letter, KEYBOARD[((kn - 21) % 12) as usize]);
        assert_eq!(note.octave, (kn as i8 - 21) / 12);
    }
}

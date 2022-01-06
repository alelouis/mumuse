use crate::messages;
use crate::music;

// Tests conversion from Midi to Note
#[test]
fn from_key_number_to_note() {
    let note_names = [
        "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
    ];
    for kn in 21..127 {
        let data_kn = messages::Data::KeyNumber(kn);
        let note = match music::Note::from_key_number(&data_kn) {
            Some(note) => note,
            None => panic!("Oups"),
        };
        assert_eq!(note.letter, note_names[((kn - 21) % 12) as usize]);
        assert_eq!(note.octave, (kn - 21) / 12);
    }
}

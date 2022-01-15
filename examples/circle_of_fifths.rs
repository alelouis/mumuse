use itertools::Itertools;
use mumuse::midi::{self, MidiSend};
use mumuse::music::common::Interval;
use mumuse::music::note::Note;

fn main() {
    // Declare root note
    let root = Note::try_from("C3").unwrap(); // can fail

    // Compute circle of fifths
    let circle_of_fifths = (0..12)
        .scan(root, |s, _| {
            *s = *s + Interval::Fifth;
            s.octave = root.octave;
            Some(*s)
        })
        .collect_vec();

    // Play them through midi
    // midi::show_output_ports(); // show output ports
    let mut conn_out = midi::get_output_connection("Virtual Midi Bus 1".to_string());
    for note in circle_of_fifths {
        note.send_midi(&mut conn_out, 100, 64);
    }
}

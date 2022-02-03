use mumuse::midi::{self, MidiSend};
use mumuse::music::chord::Chord;
use mumuse::music::note::Note;
use mumuse::music::scale::Scale;

fn main() {
    // Declare a root note
    let root = Note::try_from("C4").unwrap();

    // Fill with chords
    let chords: Vec<Chord> = vec![
        Scale::major(root).two(4),
        Scale::major(root).five(4),
        Scale::major(root).one(4),
    ];

    // Play them through midi
    // midi::show_output_ports(); // show output ports
    let mut conn_out = midi::get_output_connection("Virtual Midi Bus 1".to_string());
    for chord in chords {
        chord.send_midi_with_duration(&mut conn_out, 500, 64);
    }
}

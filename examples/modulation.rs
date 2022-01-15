use mumuse::midi::{self, MidiSend};
use mumuse::music::note::Note;
use mumuse::music::scale::Scale;

fn main() {
    // Declare a root note
    let root = Note::try_from("C4").unwrap();
    let scales = [Scale::major(root), Scale::minor(root)];

    // Play them through midi
    // midi::show_output_ports(); // show output ports
    let mut conn_out = midi::get_output_connection("Virtual Midi Bus 1".to_string());

    // Play chords in major key, then in minor
    for scale in scales {
        let chords = vec![scale.two(3), scale.five(3), scale.one(3)];
        chords
            .iter()
            .for_each(|chord| chord.send_midi(&mut conn_out, 500, 64));
    }
}

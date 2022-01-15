use mumuse::midi::{self, MidiSend};
use mumuse::music::chord::Chord;
use mumuse::music::note::Note;
use mumuse::music::scale::Scale;

fn main() {
    // Declare a root note
    let root = Note::try_from("C3").unwrap();
    let major_scale = Scale::major(root);

    // Secondary dominants
    let five_of_five = Scale::major(major_scale.notes()[5-1]).five(4); // This is V7 of V7
    let five_of_six = Scale::major(major_scale.notes()[6-1]).five(4); // This is V7 of iv7

    // Fill with chords
    let chords: Vec<Chord> = vec![
        major_scale.four(3),
        five_of_five.invert(2),
        major_scale.five(3),
        five_of_six.invert(2),
        major_scale.six(3),
    ];

    // Play them through midi
    // midi::show_output_ports(); // show output ports
    let mut conn_out = midi::get_output_connection("Virtual Midi Bus 1".to_string());
    for chord in chords {
        chord.send_midi(&mut conn_out, 500, 64);
    }
}

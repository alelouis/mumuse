use mumuse::midi::{self, MidiSend};
use mumuse::music::note::Note;
use mumuse::music::scale::Scale;

fn main() {
    // Open Midi output port connection
    let mut conn_out = midi::get_output_connection("Virtual Midi Bus 1".to_string());

    // Create root note
    let root = Note::try_from("C3").unwrap();

    // Create chord progression
    let chords = vec![
        Scale::major(root).two(4),
        Scale::major(root).five(4),
        Scale::major(root).one(4),
    ];

    // Play arp with iterators
    for chord in chords {
        // Create note iterators
        let ascend = chord.notes.clone().into_iter();
        let descend = chord.notes.clone().into_iter().rev();
        ascend
            .chain(descend)
            .for_each(|n| n.send_midi_with_duration(&mut conn_out, 100, 64));
    }
}

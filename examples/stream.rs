use mumuse::messages::Status;
use mumuse::midi;
use mumuse::music::note::Note;
use mumuse::music::stream::Stream;
use mumuse::music::time::Time;
use mumuse::music::duration::Duration;

fn main() {
    let mut stream: Stream = Stream::new();
    let mut conn_out = midi::get_output_connection("Virtual Midi Bus 1".to_string());
   
    // Constructing event stream
    let notes = ["A3", "B3", "C4", "D4", "A3", "B3", "C4", "D4"];
    let mut time = Time::new(1, 4, 1);
    let duration = Duration::new(16, 1); // 16th notes

    for n in notes {
        let note = Note::try_from(n).unwrap();
        stream.add_note(note, time, duration);
        time = time + duration;
    }

    // Real time play of events
    stream.play(&mut conn_out, 120.0, 4); // midi_connection, beat per minute, beats per bar
}

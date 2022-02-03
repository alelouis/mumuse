use mumuse::music::stream::{Stream, Event, Command};
use mumuse::messages::Status;
use mumuse::music::note::Note;
use mumuse::music::time::Time;
use mumuse::midi;

fn main() {
    let mut stream: Stream = Stream::new();
    let mut conn_out = midi::get_output_connection("Virtual Midi Bus 1".to_string());

    // Constructing event stream
    let note = Note::try_from("A3").unwrap();
    stream.add_event(Event::new(Time::new(1, 4, 1), Status::NoteOn, note));
    stream.add_event(Event::new(Time::new(1, 4, 2), Status::NoteOff, note));

    let note = Note::try_from("B3").unwrap();
    stream.add_event(Event::new(Time::new(1, 4, 2), Status::NoteOn, note));
    stream.add_event(Event::new(Time::new(1, 4, 3), Status::NoteOff, note));

    let note = Note::try_from("C3").unwrap();
    stream.add_event(Event::new(Time::new(1, 4, 3), Status::NoteOn, note));
    stream.add_event(Event::new(Time::new(1, 4, 4), Status::NoteOff, note));

    let note = Note::try_from("D3").unwrap();
    stream.add_event(Event::new(Time::new(1, 4, 4), Status::NoteOn, note));
    stream.add_event(Event::new(Time::new(2, 4, 1), Status::NoteOff, note));

    // Real time play of events
    stream.play(&mut conn_out, 120.0, 4);
}

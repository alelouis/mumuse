//! Stream of notes

// TODO: Change events time ref from u32 to Time
// TODO: Write unit tests

use crate::music::note::Note;
use crate::music::time::Time;

#[derive(Debug, Default, Clone)]
pub struct Stream {
    pub events: Vec<(Time, Event)>,
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    NoteOn(Note),
    NoteOff(Note)
}

/// Temporal arrangement of events
impl Stream {
    pub fn new() -> Self {
        Self {
            events: vec![],
        }
    }

    /// Adds event to stream
    pub fn add_event(&mut self, time: Time, event: Event) {
        self.events.push((time, event));
    }

    /// Converts Events to seconds timeline
    pub fn to_seconds(&self, bpm: f64, bpb: u32) -> Vec<(f64, Event)>{
        let bar_duration = (bpb as f64) * 60. / bpm;
        let mut events_seconds: Vec<(f64, Event)> = vec![];
        for (time, event) in self.events.iter() {
            let time_seconds = 
                bar_duration * ((time.bar-1) as f64 + (time.position-1) as f64 / time.divisions as f64);
            events_seconds.push((time_seconds, *event));
        }
        events_seconds
    }

    // Play should be done with Tokio async tick await
    // https://docs.rs/tokio/1.9.0/tokio/time/enum.MissedTickBehavior.html
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_event() {
        let mut stream: Stream = Stream::new();
        let time: Time = Time::new(1, 16, 1);
        let note: Note = Note::try_from("A3").unwrap();
        stream.add_event(time, Event::NoteOn(note));
        assert_eq!(stream.events.get(0).unwrap().0.bar, 1);
        assert_eq!(stream.events.get(0).unwrap().0.divisions, 16);
        assert_eq!(stream.events.get(0).unwrap().0.position, 1);
    }

    #[test]
    fn to_time() {
        let mut stream: Stream = Stream::new();
        let time: Time = Time::new(1, 4, 2);
        let note: Note = Note::try_from("A3").unwrap();
        stream.add_event(time, Event::NoteOn(note));
        let time_stream = stream.to_seconds(120.0, 4);
        assert_eq!(time_stream[0].0, 0.5);
        println!("{:?}", time_stream);
    }
}

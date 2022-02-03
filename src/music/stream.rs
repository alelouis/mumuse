//! Stream of notes

use crate::midi::MidiSend;
use crate::music::note::Note;
use crate::music::time::Time;
use crate::messages::Status;
use itertools::Itertools;
use tokio::time::{self, Duration};
use midir::MidiOutputConnection;

/// Midi command
#[derive(Clone, Copy, Debug)]
pub enum Command {
    NoteOn(Note),
    NoteOff(Note),
}

/// Temporal arrangement of events
#[derive(Debug, Default, Clone)]
pub struct Stream {
    pub events: Vec<Event>,
}

/// An event is a timed command
#[derive(Clone, Debug)]
pub struct Event {
    time: Time,
    status: Status,
    note: Note
}

impl Event {
    pub fn new(time: Time, status: Status, note: Note) -> Self {
        Event { time, status, note}
    }
}

impl Stream {
    /// Default constructor
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    /// Adds event to stream
    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Converts Events to seconds timeline
    pub fn to_seconds(&self, bpm: f64, bpb: u32) -> Vec<(f64, Status, Note)> {
        let bar_duration = (bpb as f64) * 60. / bpm;
        let mut events_seconds: Vec<(f64, Status, Note)> = vec![];
        for event in self.events.iter() {
            let time_seconds = bar_duration
                * ((event.time.bar - 1) as f64
                    + (event.time.position - 1) as f64 / event.time.divisions as f64);
            events_seconds.push((time_seconds, event.status, event.note));
        }
        // Sort by time
        events_seconds.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        events_seconds
    }

    /// Plays stream of events in real time
    #[tokio::main]
    pub async fn play(&self, conn_out: &mut MidiOutputConnection, bpm: f64, bpb: u32) {
        let events_seconds = self.to_seconds(bpm, bpb);
        let interval_time = 10.0; // in ms
        let mut played_events = 0;
        let mut n_tick = 0;
        let total_events = events_seconds.len();
        let mut interval = time::interval(Duration::from_millis(interval_time as u64));

        async fn play_events(
            conn_out: &mut MidiOutputConnection,
            n_tick: usize,
            interval_time: f64,
            events_seconds: &Vec<(f64, Status, Note)>,
        ) -> usize {
            let last_tick_time = n_tick as f64 * interval_time / 1000.0;
            let next_tick_time = (n_tick + 1) as f64 * interval_time / 1000.0;
            let current_events = events_seconds
                .iter()
                .filter(|event| (event.0 >= last_tick_time) && (event.0 < next_tick_time))
                .collect_vec();
            for current_event in &current_events {
                match current_event.1 {
                    Status::NoteOn => current_event.2.send_midi(Status::NoteOn, conn_out),
                    Status::NoteOff => current_event.2.send_midi(Status::NoteOff, conn_out),
                    _ => (),
                };
                println!("sent {:?}", current_event);
            }
            current_events.len()
        }

        loop {
            interval.tick().await;
            played_events += play_events(conn_out, n_tick, interval_time, &events_seconds).await;
            if total_events == played_events {
                break;
            }
            n_tick += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_event() {
        let mut stream: Stream = Stream::new();
        let time: Time = Time::new(1, 16, 1);
        let note: Note = Note::try_from("A3").unwrap();
        stream.add_event(Event::new(time, Status::NoteOn, note));
        assert_eq!(stream.events.get(0).unwrap().time.bar, 1);
        assert_eq!(stream.events.get(0).unwrap().time.divisions, 16);
        assert_eq!(stream.events.get(0).unwrap().time.position, 1);
    }

    #[test]
    fn sort_stream_time() {
        let mut stream: Stream = Stream::new();
        let note: Note = Note::try_from("A3").unwrap();

        // Unordered declaration of events
        stream.add_event(Event::new(Time::new(1, 4, 4), Status::NoteOn, note));
        stream.add_event(Event::new(Time::new(1, 4, 3), Status::NoteOn, note));
        stream.add_event(Event::new(Time::new(1, 4, 2), Status::NoteOn, note));
        stream.add_event(Event::new(Time::new(1, 4, 1), Status::NoteOn, note));

        // time_stream time should be ordered
        let time_stream = stream.to_seconds(120.0, 4);
        assert!(time_stream[0].0 < time_stream[1].0);
        assert!(time_stream[1].0 < time_stream[2].0);
        assert!(time_stream[2].0 < time_stream[3].0);
    }
}

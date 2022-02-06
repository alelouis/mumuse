//! Stream of notes

use crate::messages::Status;
use crate::midi::MidiSend;
use crate::music::duration::Duration;
use crate::music::note::Note;
use crate::music::time::Time;
use itertools::Itertools;
use midir::MidiOutputConnection;
use tokio::time::{self, Duration as TDuration};

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
    note: Note,
}

impl Event {
    pub fn new(time: Time, status: Status, note: Note) -> Self {
        Event { time, status, note }
    }
}

impl Stream {
    /// Creates a new empty `Stream` with no events.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mumuse::music::stream::Stream;
    /// let s = Stream::new(); 
    /// ```
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    /// Adds event to stream
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mumuse::music::{time::Time, note::Note, stream::{Stream, Event}};
    /// use mumuse::messages::Status;
    /// let mut stream: Stream = Stream::new();
    /// let time: Time = Time::new(1, 16, 1);
    /// let note: Note = Note::try_from("A3").unwrap();
    /// stream.add_event(Event::new(time, Status::NoteOn, note));
    /// ```
    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Adds note to stream
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mumuse::music::{time::Time, note::Note, stream::Stream, duration::Duration};
    /// let mut stream: Stream = Stream::new();
    /// let note: Note = Note::try_from("A3").unwrap();
    /// let time: Time = Time::new(1, 16, 1);
    /// let duration: Duration = Duration::new(16, 1);
    /// stream.add_note(note, time, duration);
    /// ```
    pub fn add_note(&mut self, note: Note, time: Time, duration: Duration) {
        println!("{:?} + {:?} = {:?}", time, duration, time + duration);
        self.events.push(Event::new(time, Status::NoteOn, note));
        self.events
            .push(Event::new(time + duration, Status::NoteOff, note))
    }

    /// Converts Events to seconds timeline
    ///
    /// In order to convert the stream with `Time` events, one need to declare a `bpm`
    /// (beats per minutes) and a `bpb` (beats per bar).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mumuse::music::{time::Time, note::Note, stream::Stream, duration::Duration};
    /// let mut stream: Stream = Stream::new();
    /// let note: Note = Note::try_from("A3").unwrap();
    /// let time: Time = Time::new(1, 16, 1);
    /// let duration: Duration = Duration::new(16, 1);
    /// stream.add_note(note, time, duration);
    /// let stream_seconds = stream.to_seconds(120.0, 4);
    /// ```
    pub fn to_seconds(&self, bpm: f64, bpb: u32) -> Vec<(f64, Status, Note)> {
        let mut events_seconds: Vec<(f64, Status, Note)> = vec![];
        for event in self.events.iter() {
            events_seconds.push((event.time.to_seconds(bpm, bpb), event.status, event.note));
        }
        // Sort by time
        events_seconds.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        events_seconds
    }

    /// Plays stream of events in real time
    ///
    /// Use ticking for playing the stream of events at regular intervals.
    /// For each tick, the events with Time to seconds lying in the tick window are
    /// sent as MIDI. 
    /// The `bpm` (beats per minutes) and `bpb` (beats per bar) are needed for the conversion
    /// to seconds.
    #[tokio::main]
    pub async fn play(&self, conn_out: &mut MidiOutputConnection, bpm: f64, bpb: u32) {
        let events_seconds = self.to_seconds(bpm, bpb); // Vector of events with seconds unit
        let interval_time = 10.0; // in ms
        let mut played_events = 0; // Count of sent event
        let mut n_tick = 0; // Tick number counter
        let total_events = events_seconds.len(); // Number of total events to send
        let mut interval = time::interval(TDuration::from_millis(interval_time as u64)); 

        // Async function to send midi events with constant tick time
        // Constant tick time is managed by Tokio interval ticking with Burst missed tick strategy
        // Tick time is set to 10ms here, adjust for more / less precision in event timing
        async fn play_events(
            conn_out: &mut MidiOutputConnection,
            n_tick: usize,
            interval_time: f64,
            events_seconds: &Vec<(f64, Status, Note)>,
        ) -> usize {
            let last_tick_time = n_tick as f64 * interval_time / 1000.0; // in sec
            let next_tick_time = (n_tick + 1) as f64 * interval_time / 1000.0; // in sec

            // Filter out events that to not belong to current tick window
            // Could be smarter by removing past events
            let current_events = events_seconds
                .iter()
                .filter(|event| (event.0 >= last_tick_time) && (event.0 < next_tick_time))
                .collect_vec();

            
            // Send events, currently only NoteOn and Note Off are handled.
            // Add match arms for additional case handling
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
            // We tick until all events are sent
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

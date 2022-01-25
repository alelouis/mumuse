//! Stream of notes

use std::collections::HashMap;
use crate::music::note::Note;

#[derive(Debug, Default, Clone)]
pub struct Stream {
    pub events: HashMap<u32, Vec<Event>>,
}

#[derive(Debug, Clone)]
pub enum Event {
    NoteOn(Note),
    NoteOff(Note)
}

/// Temporal arrangement of notes
impl Stream {
    fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }

    /// Adds event to stream
    fn add_event(&mut self, time: u32, event: Event) {
        match self.events.get_mut(&time) {
            Some(v) => v.push(event),
            None => {self.events.insert(time, vec![event]);}
        }
    }

    /// Adds note to stream (NoteOn and NoteOff)
    fn add_note(&mut self, time: u32, duration: u32, note: Note) {
        self.add_event(time, Event::NoteOn(note));
        self.add_event(time+duration, Event::NoteOff(note));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut stream: Stream = Stream::new();
        stream.add_note(0, 2, Note::try_from("A3").unwrap());
        stream.add_note(0, 3, Note::try_from("G3").unwrap());
        println!("{:?}", stream);
    }
}

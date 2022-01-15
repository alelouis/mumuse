extern crate mumuse;

use mumuse::music::chord::Chord;
use mumuse::music::common::Interval;
use mumuse::music::note::Note;
use mumuse::music::scale::Scale;

fn main() {
    // Declare a root note
    let root = Note::try_from("C0").unwrap();

    // Declare vector of intervals
    let intervals = vec![Interval::MajorSecond, Interval::Fifth];

    // Declare scale from root and intervals
    let scale = Scale::new(root, intervals);

    // Get notes of scale
    let notes = scale.notes();
    println!("{:?}", notes);

    // Construct Chord from Scale degree
    let one_chord = Scale::major(root).one();
    let two_chord = Scale::major(root).two();
    println!("I chord of C Major Scale : {}", one_chord);
    println!("ii chord of C Major Scale : {}", two_chord);
}

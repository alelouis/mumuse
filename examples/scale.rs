extern crate mumuse;

use mumuse::music::note::Note;
use mumuse::music::common::Interval;
use mumuse::music::scale::Scale;

fn main() {
    // Declare a root note
    let root = Note::from_str("C0").unwrap();

    // Declare vector of intervals
    let intervals = vec![Interval::MajorSecond, Interval::Fifth];

    // Declare scale from root and intervals
    let scale = Scale::new(root, intervals);

    // Get notes of scale
    let notes = scale.notes();
    println!("{:?}", notes);
}
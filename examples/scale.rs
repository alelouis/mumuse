extern crate mumuse;

use mumuse::music::note::Note;
use mumuse::music::common::Interval;
use mumuse::music::scale::Scale;

fn main() {
    let root = Note::from_str("C0").unwrap();
    let intervals = vec![Interval::MajorSecond, Interval::Fifth];
    let scale = Scale::new(root, intervals);
    let notes = scale.notes();
    println!("{:?}", notes);
}